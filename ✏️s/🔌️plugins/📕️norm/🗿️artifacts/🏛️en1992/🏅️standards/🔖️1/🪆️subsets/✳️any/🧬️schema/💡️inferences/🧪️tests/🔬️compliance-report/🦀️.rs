use crate::document::CheckStatus;
use crate::standards::v1::subsets::any::schema::inferences::evaluate;
use crate::En1992Snapshot;

#[test]
fn default_evaluate_covers_parts() {
    let report = evaluate(&En1992Snapshot::default());
    assert!(report.checks.len() >= 10);
    let parts: std::collections::BTreeSet<_> = report.checks.iter().map(|c| c.part.as_str()).collect();
    assert!(parts.iter().any(|p| p.contains("1992-1-1")));
}

#[test]
fn failing_example_not_compliant() {
    let report = evaluate(&En1992Snapshot::failing_under_reinforced());
    assert!(!report.complies());
    assert!(report.summary.fail >= 3);
    assert!(report.checks.iter().any(|c| c.status == CheckStatus::Fail && !c.remedies.is_empty()));
}
