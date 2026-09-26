use crate::En1995Snapshot;
use crate::standards::v1::subsets::any::schema::inferences::evaluate;

#[test]
fn compliant_report_complies() {
    let report = evaluate(&En1995Snapshot::compliant_building_beam());
    assert!(report.complies());
    assert!(!report.checks.is_empty());
}

#[test]
fn noncompliant_report_fails_with_remedies() {
    let report = evaluate(&En1995Snapshot::noncompliant_building());
    assert!(!report.complies());
    assert!(report.failing().all(|c| !c.remedies.is_empty()));
}
