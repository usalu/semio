//! 🌿️ Regenerates one gbXML witness through the production exporter.
use std::{env, fs, path::PathBuf};
use bim_gbxml::standards::v1::subsets::any::io::export::gbxml::export_gbxml;
use bim_snapshot::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;

fn main() {
    let args: Vec<_> = env::args_os().skip(1).collect();
    let input = fs::read_to_string(&args[0]).unwrap();
    let model = decode_model_snapshot_json(&input).unwrap();
    let path = PathBuf::from(&args[1]);
    let before = fs::read(&path).ok();
    let (document, notes) = export_gbxml(&model).unwrap();
    assert_eq!(fs::read(&path).ok(), before, "fixture changed during export");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, document).unwrap();
    println!("{}: {} notes", path.display(), notes.len());
}
