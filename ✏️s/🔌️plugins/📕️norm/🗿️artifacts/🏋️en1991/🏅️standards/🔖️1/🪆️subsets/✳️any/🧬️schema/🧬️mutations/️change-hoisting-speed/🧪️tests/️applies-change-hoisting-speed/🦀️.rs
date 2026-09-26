//! Scenario for `change-hoisting-speed`.
#[test]
fn applies_change_hoisting_speed() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
