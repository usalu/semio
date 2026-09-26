//! Scenario for `change-assumed-bridge-udl`.
#[test]
fn applies_change_assumed_bridge_udl() {
    let base = crate::En1991Snapshot::default();
    let _ = base.assumed_bridge_udl;
}
