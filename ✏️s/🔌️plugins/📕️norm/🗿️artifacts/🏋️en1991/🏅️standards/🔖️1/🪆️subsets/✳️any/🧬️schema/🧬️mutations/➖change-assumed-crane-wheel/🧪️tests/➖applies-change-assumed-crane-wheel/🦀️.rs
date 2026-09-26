//! Scenario for `change-assumed-crane-wheel`.
#[test]
fn applies_change_assumed_crane_wheel() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
