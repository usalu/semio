#[test]
fn evaluate_default_complies() {
    let doc = semio_s_artifact_norm_en1996_model::En1996Snapshot::default();
    let report = crate::evaluate(&doc);
    assert!(report.checks.len() >= 5);
    assert!(report.complies());
}

#[test]
fn evaluate_noncompliant_has_fails() {
    let doc = semio_s_artifact_norm_en1996_model::En1996Snapshot::noncompliant_multi_fail();
    let report = crate::evaluate(&doc);
    assert!(!report.complies());
    assert!(report.summary.fail >= 3);
}
