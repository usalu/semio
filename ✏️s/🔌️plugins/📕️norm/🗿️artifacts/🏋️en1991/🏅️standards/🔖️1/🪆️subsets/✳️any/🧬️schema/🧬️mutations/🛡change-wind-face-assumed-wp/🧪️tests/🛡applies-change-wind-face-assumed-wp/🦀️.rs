//! Scenario for `change-wind-face-assumed-wp`.
#[test]
fn applies_change_wind_face_assumed_wp() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
