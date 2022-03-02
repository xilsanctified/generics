struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };

    println!("The value of x and y: {} {}", integer.x(), integer.y());

    let float = Point { x: 1.1, y: 4.3 };

    println!("The value of x and y: {} {}", float.x(), float.y());
    println!("Distance from 0,0: {}", float.distance_from_origin());

    let integer_and_float = Point { x: 1.5, y: 10 };

    println!(
        "The value of x and y: {} {}",
        integer_and_float.x(),
        integer_and_float.y()
    );
}

/*
fn largest<T>(list_of_numbers: &[T]) -> T {
    let mut result = list_of_numbers[0];
    for &number in list_of_numbers {
        if number > result {
            result = number;
        }
    }
    result
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);

    println!("The largest number is: {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);

    println!("The largest number is: {}", result);
}
*/
