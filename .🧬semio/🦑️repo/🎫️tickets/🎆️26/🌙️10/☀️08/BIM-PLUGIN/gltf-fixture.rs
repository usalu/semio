use std::io::{self, Read, Write};
use bim_gltf::standards::v1::subsets::any::io::export::gltf::export_glb;
use bim_snapshot::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;

fn main() {
    let mut source = String::new();
    io::stdin().read_to_string(&mut source).unwrap();
    let snapshot = decode_model_snapshot_json(&source).unwrap();
    let (bytes, notes) = export_glb(&snapshot).unwrap();
    eprintln!("{} export notes", notes.len());
    io::stdout().lock().write_all(&bytes).unwrap();
}
