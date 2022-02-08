use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fib(n: u16) -> u64 {
	let mut pair = (0u64, 1u64, 1u64);

	match n {
		0 => 0,
		1 => 1,
		_ => {
			for _ in 1..n {
				pair.2 = pair.0 + pair.1;
				pair.0 = pair.1;
				pair.1 = pair.2;
			}
			pair.2 as u64
		}
	}
}
