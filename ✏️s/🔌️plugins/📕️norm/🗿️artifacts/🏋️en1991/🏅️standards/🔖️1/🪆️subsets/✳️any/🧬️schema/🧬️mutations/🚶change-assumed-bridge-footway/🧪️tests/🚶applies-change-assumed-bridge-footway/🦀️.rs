//! Scenario for `change-assumed-bridge-footway`.
#[test]
fn applies_change_assumed_bridge_footway() {
    let base = crate::En1991Snapshot::default();
    let _ = base.assumed_bridge_footway;
}
