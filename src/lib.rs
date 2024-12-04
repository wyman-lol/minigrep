use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // 当前二进制文件路径先消费掉
        args.next();

        let (query, file_path, ignore_case) = Self::parse_arguments(args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
    fn parse_arguments(
        mut args: impl Iterator<Item = String>,
    ) -> Result<(String, String, bool), &'static str> {
        let mut query: String = String::new();
        let mut file_path: String = String::new();
        let mut ignore_case: bool = false;

        match env::var("IGNORE_CASE").ok() {
            None => {}
            Some(arg) if arg == "0" => {
                ignore_case = false;
            }
            Some(arg) if arg == "1" => {
                ignore_case = true;
            }
            Some(_) => {
                return Err(
                    "Not support environment parse, -i or --ignore_case only support '0' or '1'!",
                )
            }
        }

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-q" | "--query" => {
                    if let Some(q) = args.next() {
                        query = q;
                    } else {
                        return Err("Didn't get a query string!");
                    }
                }
                "-f" | "--file_path" => {
                    if let Some(f) = args.next() {
                        file_path = f;
                    } else {
                        return Err("Didn't get a file path!");
                    }
                }
                "-i" | "--ignore_case" => {
                    if let Some(i) = args.next() {
                        // 命令行参数应该比环境变量参数优先才对
                        match i.as_str() {
                            "0" => {
                                ignore_case = false;
                            }
                            "1" => {
                                ignore_case = true;
                            }
                            _ => return Err(
                                "Not support command parse, -i or --ignore_case only support '0' or '1'!",
                            ),
                        }
                    }
                }
                _ => return Err("Illegal arguments!"),
            }
        }
        Ok((query, file_path, ignore_case))
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // println!("With text:\n{contents}");
    let lines = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in lines {
        println!("{line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }
    results
}
