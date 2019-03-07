use regex::Regex;
use std::iter::Map;

fn main() {
    let input = "/Users/tushar.tyagi/coding/rust/sandbox/src/input.csv";
    let pattern = Regex::new(r"(\w+), (\d+), (\w+), (\w+)").unwrap();
    let mut persons = Vec::new();

    match std::fs::read_to_string(input) {
        Ok(data) => {
            for line in data.split("\n") {
                let matches = pattern.captures(line).unwrap();
                let person = Person::new(String::from(&matches[1]), matches[2].parse::<u32>().unwrap(), String::from(&matches[3]), String::from(&matches[4]));
                
                persons.push(person);
            }

        }
        Err(err) => panic!(err)
            
    }

    println!("Finally able to decode {} records", persons.len());
    
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    residence: String,
    location: String
}

impl Person {
    fn new(name: String, age: u32, residence: String, location: String) -> Person {
        Person {
            name: name,
            age: age,
            residence: residence,
            location: location
        }
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}
