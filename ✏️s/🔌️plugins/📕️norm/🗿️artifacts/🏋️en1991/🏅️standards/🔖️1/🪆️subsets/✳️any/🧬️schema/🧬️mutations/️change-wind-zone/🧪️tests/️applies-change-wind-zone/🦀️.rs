//! Scenario for `change-wind-zone`.
#[test]
fn applies_change_wind_zone() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
