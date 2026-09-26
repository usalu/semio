//! Package-level smoke tests for EN 1990 subject schema.

#[semio_framework_async_macros::async_test]
async fn default_snapshot_has_hierarchical_subject() {
    let snapshot = crate::En1990Snapshot::default();
    assert!(!snapshot.permanents.is_empty());
    assert!(!snapshot.variables.is_empty());
    assert!(!snapshot.members.is_empty());
    assert!(!snapshot.effects.is_empty());
    assert_eq!(snapshot.consequence_class, 2);
}

#[semio_framework_async_macros::async_test]
async fn package_descriptor_parses() {
    let _ = crate::package_descriptor().expect("package descriptor");
}
