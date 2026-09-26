//! Scenario for `change-bridge-lane`.
#[test]
fn applies_change_bridge_lane() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
