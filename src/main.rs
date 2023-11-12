use dialoguer::{theme::ColorfulTheme, Select};
use std::io;

fn main() {
    println!("Temperature converter!");

    let selections = ["Celsius => Farenheit", "Farenheit => Celsius"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which conversion would you like to perform today?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Celsius => Farenheit" => println!("Enter the temperature in celsius"),
        "Farenheit => Celsius" => println!("Enter the temperature in farenheit"),
        _ => unreachable!(),
    }

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read input");

    let temp: f64 = temp.trim().parse().expect("Not a valid temperature");

    let response = match selections[selection] {
        "Celsius => Farenheit" => celsius_to_farenheit(temp),
        "Farenheit => Celsius" => farenheit_to_celsius(temp),
        _ => unreachable!(),
    };

    let response_type = match selections[selection] {
        "Celsius => Farenheit" => "farenheit",
        "Farenheit => Celsius" => "celsius",
        _ => unreachable!(),
    };

    println!("the temperature in {} is {:.2}", response_type, response)
}

fn celsius_to_farenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32 as f64
}

fn farenheit_to_celsius(farenheit: f64) -> f64 {
    (farenheit - 32 as f64) / 1.8
}
