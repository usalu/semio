//! Scenario for `remove-wind-faces`.
#[test]
fn applies_remove_wind_faces() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
