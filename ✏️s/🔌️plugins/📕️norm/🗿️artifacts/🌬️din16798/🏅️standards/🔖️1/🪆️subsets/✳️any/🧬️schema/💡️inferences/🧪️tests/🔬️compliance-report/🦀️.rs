//! 🔬️ End-to-end compliance report for hierarchical din16798 subjects.

use crate::standards::v1::subsets::any::schema::inferences::evaluate;
use crate::Din16798Snapshot;

#[test]
fn compliant_office_evaluate_complies() {
    let report = evaluate(&Din16798Snapshot::compliant_office());
    assert!(report.complies());
    assert!(report.summary.total > 10);
}

#[test]
fn noncompliant_office_evaluate_fails_across_parts() {
    let report = evaluate(&Din16798Snapshot::noncompliant_office());
    assert!(!report.complies());
    let parts: std::collections::BTreeSet<_> = report.failing().map(|c| c.part.as_str()).collect();
    assert!(parts.contains("DIN EN 16798-1"));
    assert!(parts.iter().any(|p| p.contains("16798-3") || p.contains("16798-5") || p.contains("16798-7") || p.contains("16798-17")), "parts={parts:?}");
}

#[test]
fn empty_zones_marks_na_not_hard_fail_only() {
    let mut doc = Din16798Snapshot::compliant_office();
    doc.zones.clear();
    let report = evaluate(&doc);
    assert!(report.summary.not_applicable >= 1);
}
