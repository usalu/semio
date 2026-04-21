use std::sync::{Arc, RwLock};

use tempfile::tempdir;

use crate::kit::KitStore;

#[test]
fn sqlite_snapshot_roundtrip() {
    let kit = Arc::new(RwLock::new(KitStore::new("sqlite-roundtrip")));
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("kit.db");
    kit.read().expect("read").save_sqlite(&path).expect("save");
    let kit2 = KitStore::load_sqlite(&path).expect("load");
    assert_eq!(
        kit.read().expect("r1").hash(),
        kit2.read().expect("r2").hash(),
        "SQLite JSON snapshot preserves hash"
    );
}
