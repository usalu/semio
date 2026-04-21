use crate::kit::KitStore;

#[test]
fn validate_empty_kit_name_fails() {
    let mut k = KitStore::new(" ");
    k.name = "  ".to_string();
    let v = k.validate();
    assert!(!v.is_valid);
    assert!(v.errors.iter().any(|e| e.contains("kit.name")));
}
