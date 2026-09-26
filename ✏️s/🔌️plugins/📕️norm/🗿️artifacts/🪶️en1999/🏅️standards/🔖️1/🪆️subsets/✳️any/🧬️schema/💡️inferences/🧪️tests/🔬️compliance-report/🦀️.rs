//! 🔬️ evaluate() report shape for hierarchical EN 1999 subject.

use crate::snapshot::En1999Snapshot;
use crate::standards::v1::subsets::any::schema::inferences::evaluate;

#[test]
fn evaluate_emits_member_and_connection_checks() {
    let report = evaluate(&En1999Snapshot::compliant_roof_purlin());
    assert!(report.checks.len() >= 8);
    assert!(report.checks.iter().any(|c| c.id.contains("6.3.1")));
    assert!(report.complies(), "worst={} fails={:?}", report.worst_utilization(), report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
}

#[test]
fn noncompliant_evaluate_fails() {
    let report = evaluate(&En1999Snapshot::noncompliant_multi_fail());
    assert!(!report.complies());
    assert!(report.failing().next().unwrap().remedies.len() >= 1);
}
