use std::sync::{Arc, RwLock};

use crate::kit::KitStore;

#[test]
fn kit_name_change_recomputes_validation() {
    let kit = Arc::new(RwLock::new(KitStore::new("ok")));
    assert!(kit.read().expect("r").validate().is_valid);
    kit.write().expect("w").set_name("   ".to_string());
    assert!(!kit.read().expect("r").validate().is_valid);
}
