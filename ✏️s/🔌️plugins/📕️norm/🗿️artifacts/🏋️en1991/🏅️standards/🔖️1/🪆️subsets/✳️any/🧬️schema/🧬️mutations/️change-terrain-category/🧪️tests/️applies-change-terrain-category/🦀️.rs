//! Scenario for `change-terrain-category`.
#[test]
fn applies_change_terrain_category() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
