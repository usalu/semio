//! Scenario for `change-mixed-terrain-upwind`.
#[test]
fn applies_change_mixed_terrain_upwind() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
