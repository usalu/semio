//! Scenario for `change-altitude`.
#[test]
fn applies_change_altitude() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
