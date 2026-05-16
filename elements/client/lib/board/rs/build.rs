//! @emoji 🎨 Emits `OUT_DIR/elements_styling_board.rs` from `elements/core/styling/tokens.json` for Vello `Color` constants.
use serde_json::Value;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
	let manifest = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
	let tokens_path = Path::new(&manifest).join("../../../../core/styling/tokens.json");
	println!("cargo:rerun-if-changed={}", tokens_path.display());
	let raw = fs::read_to_string(&tokens_path).unwrap_or_else(|e| panic!("read {}: {e}", tokens_path.display()));
	let root: Value = serde_json::from_str(&raw).expect("parse tokens.json");
	let canvas = root
		.get("board_vello_canvas")
		.and_then(Value::as_object)
		.expect("board_vello_canvas must be an object");
	let mut keys: Vec<&String> = canvas.keys().collect();
	keys.sort();
	let out_path = Path::new(&env::var("OUT_DIR").expect("OUT_DIR")).join("elements_styling_board.rs");
	let mut f = fs::File::create(&out_path).expect("create elements_styling_board.rs");
	for key in keys {
		let val = &canvas[key];
		let ident = key.to_ascii_uppercase();
		let arr = val.as_array().unwrap_or_else(|| panic!("{key}: expected array"));
		let n: Vec<f64> = arr
			.iter()
			.map(|x| x.as_f64().unwrap_or_else(|| panic!("{key}: non-numeric")))
			.collect();
		assert!(n.len() == 4, "{key}: expected 4 floats");
		let a = rust_f32_lit(n[0]);
		let b = rust_f32_lit(n[1]);
		let c = rust_f32_lit(n[2]);
		let d = rust_f32_lit(n[3]);
		writeln!(f, "pub const {ident}: Color = Color::new([{a}, {b}, {c}, {d}]);").expect("write");
	}
}

fn rust_f32_lit(x: f64) -> String {
	format!("{:?}_f32", x as f32)
}
