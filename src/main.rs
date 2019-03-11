use std::io;

fn main() {
    println!("***CONVERTION BETWEEN FAHRENHEIT AND CELCIUS***");
    let (base_unit, quote_unit) = get_base_and_quote_units();
    let temp = gettemp();
    let converted_temp = convert(base_unit, &temp);

    // 1st decimal place formatting with {:.1}
    println!("RESULT: {}{} -> {:.1}{}", temp, base_unit, converted_temp, quote_unit);
}

fn get_base_and_quote_units() -> (char, char) {
    loop {
        println!("What is your base unit, F(ahrenheit) or C(elcius)?");

        let mut f_or_c = String::new();

        io::stdin().read_line(&mut f_or_c).expect("Failed to read the line");

        if f_or_c == "\n" {
            //Only new line character supplied
            println!("No input supplied! Please type either F(ahrenheit) or C(elcius)!");
        } else if f_or_c.len() > 2 {
            //'\n' will be added at the end as user press Enter key!
            //Thus, 1 (one) char plus '\n'
            println!("Please type ONLY F(ahrenheit) or C(elcius)!");
        } else {
            match f_or_c.trim() {
                "f" | "F" => {
                    println!("Your base is: Fahrenheit(F)");
                    return ('F', 'C');
                },
                "c" | "C" => {
                    println!("Your base is: Celcius(C)");
                    return ('C', 'F');
                },
                _   => println!("Sorry, don't understand your input!"),
            }
        }

        // if f_or_c.len() > 2 {
        //     //'\n' will be added at the end as user press Enter key!
        //     //Thus, 1 (one) char plus '\n'
        //     println!("Please type either F(ahrenheit) or C(elcius)!");
        // } else {
        //     match f_or_c.trim() {
        //         "f" | "F" => {
        //             println!("Your base is: Fahrenheit(F)");
        //             return ('F', 'C');
        //         },
        //         "c" | "C" => {
        //             println!("Your base is: Celcius(C)");
        //             return ('C', 'F');
        //         },
        //         _   => println!("Sorry, don't understand your input!"),
        //     }
        // }
    }
}

fn gettemp() -> f32 {
    loop {
        println!("What is your temperature?");

        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read the line");

        match temp.trim().parse() {
        //Parse signature is pub fn parse<F>(&self) -> Result<F, <F as FromStr>::Err> where F: FromStr,
        //compiler knows it must be Result<f32, ...>
            Ok(temp) => return temp,
            Err(_) => println!("Not a number!"),
        }
    }
}

fn convert(unit: char, temp: &f32) -> f32 {
    match unit {
        'F' => (temp - 32.0) * 5.0 / 9.0,
        'C' => (temp * 9.0 / 5.0) + 32.0,
        _  => 0.0,
    }
}
