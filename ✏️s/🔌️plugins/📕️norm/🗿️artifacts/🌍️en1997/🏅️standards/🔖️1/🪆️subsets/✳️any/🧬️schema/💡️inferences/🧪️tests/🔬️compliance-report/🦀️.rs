//! 🔬 Compliance report projection tests.

use crate::evaluate;
use semio_s_artifact_norm_en1997_model::{compliant_demo, noncompliant_demo};

#[test]
fn evaluate_default_has_checks() {
    let report = evaluate(&compliant_demo());
    assert!(report.checks.len() >= 8);
    assert!(report.complies());
}

#[test]
fn evaluate_noncompliant_fails() {
    let report = evaluate(&noncompliant_demo());
    assert!(!report.complies());
    assert!(report.summary.fail >= 4);
}
