use std::{ fs, env, error::Error};

pub struct Config{
  pub  query: String,
   pub filename: String,
   pub case_sensitive: bool,
}

impl Config {
   pub fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
           return Err("Incomplete arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    
    }
}


pub fn run(values: Config) -> Result<(), Box<dyn Error>>{
   let contents = fs::read_to_string(values.filename)?;

   if values.case_sensitive {
    for line in search(&values.query, &contents){
        println!("{}", line)
    }
   }else {
    for line in search_case_insensitive(&values.query, &contents){
        println!("{}", line)
    }
   }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line)
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();

    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_search(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive(){
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