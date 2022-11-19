use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn read_file_into_int_list() -> io::Result<Vec<i32>>{
    // https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust
    let file = File::open("src/day1_data/simple_input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut values: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let value: i32 = line.unwrap().parse::<i32>().unwrap();
        values.push(value);
    }

    Ok(values)
}

fn number_of_depth_increases(depths: Vec<i32>) -> i32{
    let mut last_depth = i32::MAX;
    let mut depth_increase_counter = 0;
    for depth in depths {
        println!("comparing {last_depth} and {depth}");
        if depth > last_depth{
            depth_increase_counter += 1;
        }
        last_depth = depth;
    }
    return depth_increase_counter;
}

fn main() {
    let values: Vec<i32> = read_file_into_int_list()
        .expect("Failed to interpret file");

    let depth_increases: i32 = number_of_depth_increases(values);

    println!("{depth_increases}");
}

