use crate::design::DesignStore;

#[test]
fn flatten_map_empty_design() {
    let d = DesignStore::new("x");
    let m = d.flatten_map();
    assert!(m.is_empty());
}
