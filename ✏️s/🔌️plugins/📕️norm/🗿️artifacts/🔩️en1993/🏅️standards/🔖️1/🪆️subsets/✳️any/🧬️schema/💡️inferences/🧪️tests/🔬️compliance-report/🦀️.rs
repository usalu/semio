//! 🔬️ evaluate() report shape for hierarchical EN 1993 subject.

use crate::document::CheckStatus;
use crate::standards::v1::subsets::any::schema::inferences::evaluate;
use crate::En1993Snapshot;

#[test]
fn evaluate_compliant_covers_core_parts_and_passes() {
    let report = evaluate(&En1993Snapshot::compliant_heb240_frame());
    assert!(report.checks.len() >= 8);
    assert!(report.complies());
    let parts: Vec<_> = report.by_part().into_iter().map(|(p, _)| p).collect();
    assert!(parts.iter().any(|p| p.contains("1993-1-1")));
    assert!(parts.iter().any(|p| p.contains("1993-1-8")));
}

#[test]
fn evaluate_noncompliant_has_localized_fail_remedies() {
    let report = evaluate(&En1993Snapshot::noncompliant_overloaded_frame());
    assert!(!report.complies());
    for check in report.failing() {
        assert!(!check.title.en.is_empty());
        assert!(!check.title.de.is_empty());
        assert!(!check.explanation.en.is_empty());
        assert!(!check.explanation.de.is_empty());
        assert!(!check.remedies.is_empty());
        assert!(!check.remedies[0].action.en.is_empty());
        assert!(!check.remedies[0].action.de.is_empty());
    }
    assert!(report.checks.iter().any(|c| c.status == CheckStatus::Fail && c.id.contains("6.3.1")));
}
