//! Scenario for `change-assumed-bridge-lm2`.
#[test]
fn applies_change_assumed_bridge_lm2() {
    let base = crate::En1991Snapshot::default();
    let _ = base.assumed_bridge_lm2;
}
