//! Scenario for `change-bridge-span`.
#[test]
fn applies_change_bridge_span() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
