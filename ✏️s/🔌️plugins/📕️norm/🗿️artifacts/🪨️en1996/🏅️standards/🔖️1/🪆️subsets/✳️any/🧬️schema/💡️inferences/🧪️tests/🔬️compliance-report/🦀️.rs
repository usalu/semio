#[test]
fn evaluate_default_complies() {
    let doc = crate::En1996Snapshot::default();
    let report = super::super::evaluate(&doc);
    assert!(report.checks.len() >= 5);
    assert!(report.complies());
}

#[test]
fn evaluate_noncompliant_has_fails() {
    let doc = crate::En1996Snapshot::noncompliant_multi_fail();
    let report = super::super::evaluate(&doc);
    assert!(!report.complies());
    assert!(report.summary.fail >= 3);
}
