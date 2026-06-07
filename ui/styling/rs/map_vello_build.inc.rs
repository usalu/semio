// @emoji 🗺️ Emits `OUT_DIR/elements_styling_map.rs` — Vello paints for GIS map from `tokens.json`.
use serde_json::Value;
use std::collections::BTreeMap;
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

enum MapPaintSrc<'a> {
    Token { key: &'a str },
}

struct MapPaintRow<'a> {
    const_name: &'a str,
    src: MapPaintSrc<'a>,
    alpha: f64,
}

const MAP_PAINT_ROWS: &[MapPaintRow<'_>] = &[
    MapPaintRow {
        const_name: "SURFACE_CLEAR",
        src: MapPaintSrc::Token { key: "dark-6-7" },
        alpha: 1.0,
    },
    MapPaintRow {
        const_name: "LAND_FILL",
        src: MapPaintSrc::Token { key: "dark-5-7" },
        alpha: 1.0,
    },
    MapPaintRow {
        const_name: "LAND_STROKE",
        src: MapPaintSrc::Token { key: "gray-300" },
        alpha: 0.42,
    },
    MapPaintRow {
        const_name: "LABEL_FILL",
        src: MapPaintSrc::Token { key: "light" },
        alpha: 1.0,
    },
    MapPaintRow {
        const_name: "LABEL_HALO",
        src: MapPaintSrc::Token { key: "dark-6-7" },
        alpha: 0.92,
    },
    MapPaintRow {
        const_name: "REGION_FILL",
        src: MapPaintSrc::Token { key: "secondary" },
        alpha: 0.22,
    },
    MapPaintRow {
        const_name: "REGION_STROKE",
        src: MapPaintSrc::Token { key: "secondary" },
        alpha: 0.9,
    },
    MapPaintRow {
        const_name: "ROUTE_STROKE",
        src: MapPaintSrc::Token { key: "tertiary" },
        alpha: 0.92,
    },
    MapPaintRow {
        const_name: "POSITION_FILL",
        src: MapPaintSrc::Token { key: "primary" },
        alpha: 1.0,
    },
    MapPaintRow {
        const_name: "POSITION_STROKE",
        src: MapPaintSrc::Token { key: "light" },
        alpha: 1.0,
    },
];

pub fn emit_map_vello_styles(manifest_dir: &Path, out_dir: &Path) {
    let tokens_path = manifest_dir
        .ancestors()
        .flat_map(|dir| [dir.join("ui/styling/tokens.json"), dir.join("elements/styling/tokens.json")])
        .find(|path| path.is_file())
        .unwrap_or_else(|| manifest_dir.join("../../../ui/styling/tokens.json"));
    println!("cargo:rerun-if-changed={}", tokens_path.display());
    let raw = fs::read_to_string(&tokens_path).unwrap_or_else(|e| panic!("read {}: {e}", tokens_path.display()));
    let root: Value = serde_json::from_str(&raw).expect("parse tokens.json");
    let colors_val = root.get("colors").expect("tokens.json must have colors");
    let colors_obj = colors_val.as_object().expect("colors must be object");
    let colors: BTreeMap<String, Value> = colors_obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

    let out_path = out_dir.join("elements_styling_map.rs");
    let mut f = fs::File::create(&out_path).expect("create elements_styling_map.rs");
    writeln!(
        f,
        "// @emoji 🗺️ Auto-generated from ui/styling/tokens.json — do not edit by hand."
    )
    .expect("write");
    for row in MAP_PAINT_ROWS {
        let MapPaintSrc::Token { key } = row.src;
        let hex = color_hex_from_tokens(&colors, key);
        let (lr, lg, lb, la) = linear_rgba_from_hex(&hex, row.alpha);
        writeln!(
            f,
            "pub const {}: Color = Color::new([{}, {}, {}, {}]);",
            row.const_name,
            rust_f32_lit(lr),
            rust_f32_lit(lg),
            rust_f32_lit(lb),
            rust_f32_lit(la)
        )
        .expect("write");
    }
}
