use std::sync::{Arc, RwLock};

use crate::kit::KitStore;

#[test]
fn kit_json_roundtrip_hash_stable() {
    let kit = Arc::new(RwLock::new(KitStore::new("roundtrip-test")));
    let json = kit.read().expect("read").to_json_pretty().expect("to json");
    let kit2 = KitStore::from_json_str(&json).expect("from json");
    assert_eq!(
        kit.read().expect("read").hash(),
        kit2.read().expect("read2").hash(),
        "hash stable across JSON round-trip"
    );
}
