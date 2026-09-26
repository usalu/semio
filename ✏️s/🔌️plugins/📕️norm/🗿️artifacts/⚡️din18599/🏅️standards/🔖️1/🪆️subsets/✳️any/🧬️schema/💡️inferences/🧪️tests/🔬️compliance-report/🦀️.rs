//! 🔬️ Compliance report shape for DIN V 18599 rebuild.

use crate::document::CheckStatus;
use crate::subjects;
use crate::standards::v1::subsets::any::schema::inferences::evaluate;

#[test]
fn evaluate_emits_grouped_geg_and_part_checks() {
    let report = evaluate(&crate::subjects::compliant_detached_house());
    assert!(report.summary.total >= 8);
    assert!(report.complies());
    assert!(report.checks.iter().any(|c| c.part.contains("GEG")));
    assert!(report.checks.iter().any(|c| c.title.en.contains("Primary energy") || c.id.contains("qp")));
}

#[test]
fn noncompliant_report_has_localized_remedies() {
    let report = evaluate(&crate::subjects::noncompliant_detached_house());
    assert!(!report.complies());
    let fail = report.failing().next().unwrap();
    assert!(!fail.remedies.is_empty());
    assert!(!fail.title.de.is_empty());
    assert!(!fail.explanation.de.is_empty());
    assert!(!fail.remedies[0].action.de.is_empty());
    assert_ne!(fail.status, CheckStatus::Pass);
}
