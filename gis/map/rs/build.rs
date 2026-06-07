//! @emoji 📎 GIS map Vello palette from `ui/styling/tokens.json`.

use std::path::PathBuf;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../ui/styling/rs/map_vello_build.inc.rs"));

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));
    emit_map_vello_styles(&manifest_dir, &out_dir);
}
