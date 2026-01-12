fn main() {
    // Read the input file
    let contents =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // Split the contents into lines
    let movements: Vec<&str> = contents.lines().collect();

    // Initialize dial starting position
    let mut current_pos = 50;

    // Initialize zero counter
    let mut num_zeros = 0;

    // Iterate over the lines
    for movement in movements {
        let (new_pos, zeros_count) = turn_dial(current_pos, movement);
        current_pos = new_pos;
        num_zeros += zeros_count;
    }

    // Print the number of zeros
    println!("Number of zeros: {}", num_zeros);
}

/// Turns the dial based on the movement string and counts the number of zeros passed
///
/// ## Arguments
/// * `current_pos` - The current position of the dial
/// * `movement` - The movement string
///
/// ## Returns
/// * `(i32, 132)` - The new position of the dial and the number of zeros
fn turn_dial(current_pos: i32, movement: &str) -> (i32, i32) {
    // Process movement into direction and distance
    let (direction, distance) = movement.split_at(1);
    let distance = distance.trim().parse::<i32>().unwrap();

    // Calculate new dial position
    let (new_pos, num_zeros) = if direction == "L" {
        let new_p = ((current_pos - distance) % 100 + 100) % 100;
        let count = if current_pos == 0 {
            distance / 100
        } else if distance >= current_pos {
            1 + (distance - current_pos) / 100
        } else {
            0
        };
        (new_p, count)
    } else {
        let new_p = (current_pos + distance) % 100;
        let count = (current_pos + distance) / 100;
        (new_p, count)
    };

    // Return new dial position and number of zeros
    (new_pos, num_zeros)
}
