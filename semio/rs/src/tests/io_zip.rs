use std::sync::{Arc, RwLock};

use tempfile::tempdir;

use crate::kit::KitStore;

#[test]
fn zip_kit_json_roundtrip() {
    let kit = Arc::new(RwLock::new(KitStore::new("zip-roundtrip")));
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("kit.zip");
    kit.read().expect("read").save_zip(&path).expect("save zip");
    let kit2 = KitStore::load_zip(&path).expect("load zip");
    assert_eq!(
        kit.read().expect("r1").hash(),
        kit2.read().expect("r2").hash(),
        "zip kit.json preserves hash"
    );
}
