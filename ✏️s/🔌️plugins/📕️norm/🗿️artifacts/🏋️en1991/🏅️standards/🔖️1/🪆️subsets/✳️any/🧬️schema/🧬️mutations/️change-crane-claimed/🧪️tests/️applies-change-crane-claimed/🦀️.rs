//! Scenario for `change-crane-claimed`.
#[test]
fn applies_change_crane_claimed() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
