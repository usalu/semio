//! Scenario for `change-assumed-bridge-tandem`.
#[test]
fn applies_change_assumed_bridge_tandem() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
