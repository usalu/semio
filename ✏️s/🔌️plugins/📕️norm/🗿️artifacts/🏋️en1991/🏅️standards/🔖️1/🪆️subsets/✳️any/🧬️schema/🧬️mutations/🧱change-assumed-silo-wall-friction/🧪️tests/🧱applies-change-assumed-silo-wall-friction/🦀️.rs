//! Scenario for `change-assumed-silo-wall-friction`.
#[test]
fn applies_change_assumed_silo_wall_friction() {
    let base = crate::En1991Snapshot::default();
    let _ = base.assumed_silo_wall_friction;
}
