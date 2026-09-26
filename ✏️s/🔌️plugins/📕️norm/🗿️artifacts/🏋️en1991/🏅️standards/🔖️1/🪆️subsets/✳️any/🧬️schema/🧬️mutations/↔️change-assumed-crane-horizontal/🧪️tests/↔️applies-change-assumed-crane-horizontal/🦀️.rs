//! Scenario for `change-assumed-crane-horizontal`.
#[test]
fn applies_change_assumed_crane_horizontal() {
    let base = crate::En1991Snapshot::default();
    let _ = base.assumed_crane_horizontal;
}
