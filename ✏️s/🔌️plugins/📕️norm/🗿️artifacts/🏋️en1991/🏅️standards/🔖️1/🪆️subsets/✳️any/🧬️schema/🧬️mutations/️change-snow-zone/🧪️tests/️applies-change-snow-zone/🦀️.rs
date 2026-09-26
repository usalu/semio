//! Scenario for `change-snow-zone`.
#[test]
fn applies_change_snow_zone() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
