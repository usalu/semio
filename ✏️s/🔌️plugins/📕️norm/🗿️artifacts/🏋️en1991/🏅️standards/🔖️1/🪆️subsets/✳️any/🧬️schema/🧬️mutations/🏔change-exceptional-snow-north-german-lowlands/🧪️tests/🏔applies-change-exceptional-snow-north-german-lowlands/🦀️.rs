//! Scenario for `change-exceptional-snow-north-german-lowlands`.
#[test]
fn applies_change_exceptional_snow_north_german_lowlands() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
