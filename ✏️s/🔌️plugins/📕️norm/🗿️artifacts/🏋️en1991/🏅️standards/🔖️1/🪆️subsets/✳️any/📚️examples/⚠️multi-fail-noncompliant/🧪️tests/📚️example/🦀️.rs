use semio_s_artifact_norm_en1991_inferences::evaluate;
use semio_s_artifact_norm_en1991_io::text::snapshot::decode_en1991_dsl;

#[test]
fn primary_asset_fails_at_least_two_checks_each_with_remedies() {
    let report = evaluate(&decode_en1991_dsl(crate::multi_fail_noncompliant::PRIMARY_TEXT).expect("multi-fail-noncompliant DSL decodes"));
    let failing: Vec<_> = report.failing().collect();
    assert!(failing.len() >= 2, "expected at least 2 failing checks, got {:?}", failing.iter().map(|c| c.id.clone()).collect::<Vec<_>>());
    for check in &failing {
        assert!(!check.remedies.is_empty(), "{} has no remedy", check.id);
    }
}
