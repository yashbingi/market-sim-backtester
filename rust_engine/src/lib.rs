// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// pub fn simulate_market() {
//     println!("Simulating market...");
// }

use pyo3::prelude::*;

#[pyfunction]
fn simulate_market() {
    println!("Market simulation from Rust!");
}

#[pymodule]
fn rust_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simulate_market, m)?)?;
    Ok(())
}