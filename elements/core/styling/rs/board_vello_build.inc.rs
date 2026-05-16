// @emoji 🎨 Emits `OUT_DIR/elements_styling_board.rs` — linear-sRGB `Color` defaults for the board canvas from `tokens.json` `colors` (same keys as Tailwind `@theme` / `--color-*`).
use serde_json::Value;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn srgb_byte_to_linear_u8(c: u8) -> f64 {
	let x = (c as f64) / 255.0;
	if x <= 0.04045 {
		x / 12.92
	} else {
		((x + 0.055) / 1.055).powf(2.4)
	}
}

fn parse_hex6(s: &str) -> Option<(u8, u8, u8)> {
	let s = s.trim().strip_prefix('#')?;
	if s.len() != 6 {
		return None;
	}
	let v = u32::from_str_radix(s, 16).ok()?;
	Some((((v >> 16) & 0xff) as u8, ((v >> 8) & 0xff) as u8, (v & 0xff) as u8))
}

fn color_hex_from_tokens(colors: &BTreeMap<String, Value>, key: &str) -> String {
	colors
		.get(key)
		.and_then(|v| v.as_str())
		.unwrap_or_else(|| panic!("tokens.colors[{key:?}] missing or not a string"))
		.to_string()
}

fn linear_rgba_from_hex(hex: &str, alpha: f64) -> (f64, f64, f64, f64) {
	let (r, g, b) = parse_hex6(hex).unwrap_or_else(|| panic!("invalid hex {hex:?}"));
	(
		srgb_byte_to_linear_u8(r),
		srgb_byte_to_linear_u8(g),
		srgb_byte_to_linear_u8(b),
		alpha,
	)
}

fn rust_f32_lit(x: f64) -> String {
	format!("{:?}_f32", x as f32)
}

enum BoardPaintSrc<'a> {
	Token { key: &'a str },
	Hex { hex: &'a str },
}

struct BoardPaintRow<'a> {
	const_name: &'a str,
	src: BoardPaintSrc<'a>,
	alpha: f64,
}

const BOARD_PAINT_ROWS: &[BoardPaintRow<'_>] = &[
	BoardPaintRow {
		const_name: "RASTER_CLEAR",
		src: BoardPaintSrc::Token { key: "light" },
		alpha: 1.0,
	},
	BoardPaintRow {
		const_name: "GRID_MINOR_STROKE",
		src: BoardPaintSrc::Token { key: "gray" },
		alpha: 0.22,
	},
	BoardPaintRow {
		const_name: "EDGE_STROKE",
		src: BoardPaintSrc::Token { key: "gray" },
		alpha: 1.0,
	},
	BoardPaintRow {
		const_name: "EDGE_STROKE_SELECTED",
		src: BoardPaintSrc::Token { key: "primary" },
		alpha: 1.0,
	},
	BoardPaintRow {
		const_name: "NODE_FILL",
		src: BoardPaintSrc::Token { key: "l-l-l-g" },
		alpha: 1.0,
	},
	BoardPaintRow {
		const_name: "NODE_STROKE",
		src: BoardPaintSrc::Token { key: "dark" },
		alpha: 1.0,
	},
	BoardPaintRow {
		const_name: "NODE_FILL_SELECTED",
		src: BoardPaintSrc::Hex { hex: "#f0c8cc" },
		alpha: 1.0,
	},
	BoardPaintRow {
		const_name: "NODE_STROKE_SELECTED",
		src: BoardPaintSrc::Token { key: "primary" },
		alpha: 1.0,
	},
	BoardPaintRow {
		const_name: "HANDLE_FILL",
		src: BoardPaintSrc::Token { key: "light" },
		alpha: 1.0,
	},
	BoardPaintRow {
		const_name: "HANDLE_STROKE",
		src: BoardPaintSrc::Token { key: "dark" },
		alpha: 1.0,
	},
	BoardPaintRow {
		const_name: "HANDLE_FILL_SELECTED",
		src: BoardPaintSrc::Token { key: "primary" },
		alpha: 1.0,
	},
	BoardPaintRow {
		const_name: "HANDLE_STROKE_SELECTED",
		src: BoardPaintSrc::Token { key: "primary" },
		alpha: 1.0,
	},
	BoardPaintRow {
		const_name: "SELECTION_PREVIEW_FILL",
		src: BoardPaintSrc::Token { key: "primary" },
		alpha: 0.14,
	},
	BoardPaintRow {
		const_name: "SELECTION_PREVIEW_STROKE",
		src: BoardPaintSrc::Token { key: "primary" },
		alpha: 0.75,
	},
];

fn main() {
	let manifest = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
	let tokens_path = Path::new(&manifest).join("../../../../core/styling/tokens.json");
	println!("cargo:rerun-if-changed={}", tokens_path.display());
	let raw = fs::read_to_string(&tokens_path).unwrap_or_else(|e| panic!("read {}: {e}", tokens_path.display()));
	let root: Value = serde_json::from_str(&raw).expect("parse tokens.json");
	let colors_val = root.get("colors").expect("tokens.json must have colors");
	let colors_obj = colors_val.as_object().expect("colors must be object");
	let colors: BTreeMap<String, Value> = colors_obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

	let out_path = Path::new(&env::var("OUT_DIR").expect("OUT_DIR")).join("elements_styling_board.rs");
	let mut f = fs::File::create(&out_path).expect("create elements_styling_board.rs");

	for row in BOARD_PAINT_ROWS {
		let hex = match row.src {
			BoardPaintSrc::Token { key } => color_hex_from_tokens(&colors, key),
			BoardPaintSrc::Hex { hex } => hex.to_string(),
		};
		let (lr, lg, lb, la) = linear_rgba_from_hex(&hex, row.alpha);
		let a = rust_f32_lit(lr);
		let b = rust_f32_lit(lg);
		let c = rust_f32_lit(lb);
		let d = rust_f32_lit(la);
		writeln!(
			f,
			"pub const {}: Color = Color::new([{}, {}, {}, {}]);",
			row.const_name, a, b, c, d
		)
		.expect("write");
	}
}
