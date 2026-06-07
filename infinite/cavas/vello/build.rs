//! 🔧 Build-time codegen for icon shortcode lookup table.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let json_path = manifest_dir.join("assets/shortcodes.json");
    println!("cargo:rerun-if-changed={}", json_path.display());

    let raw = fs::read_to_string(&json_path).expect("read shortcodes.json");
    let map: BTreeMap<String, String> = serde_json::from_str(&raw).expect("parse shortcodes.json");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let mut out = String::from("// Generated shortcode → emoji lookup.\n\n");
    out.push_str("pub fn icon_shortcode_to_emoji(code: &str) -> Option<&'static str> {\n");
    out.push_str("    match code {\n");
    for (code, emoji) in &map {
        out.push_str(&format!("        {code:?} => Some({emoji:?}),\n"));
    }
    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n");

    fs::write(out_dir.join("icon_shortcode_match.rs"), out).expect("write icon_shortcode_match.rs");
}
