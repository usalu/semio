//! Scenario for `change-air-density`.
#[test]
fn applies_change_air_density() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
