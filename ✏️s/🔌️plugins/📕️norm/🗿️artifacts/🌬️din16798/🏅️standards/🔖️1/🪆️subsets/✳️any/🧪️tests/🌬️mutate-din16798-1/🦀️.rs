//! 🧪 Hierarchical DIN EN 16798 mutate smoke.
use crate::Din16798Snapshot;

#[semio_framework_async_macros::async_test]
async fn compliant_office_snapshot_has_zone_and_vent() {
    let s = Din16798Snapshot::compliant_office();
    assert!(!s.zones.is_empty());
    assert!(!s.vent_systems.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn noncompliant_office_fails_evaluate() {
    let s = Din16798Snapshot::noncompliant_office();
    let report = crate::artifact_schema::inferences::evaluate(&s);
    assert!(!report.complies());
}
