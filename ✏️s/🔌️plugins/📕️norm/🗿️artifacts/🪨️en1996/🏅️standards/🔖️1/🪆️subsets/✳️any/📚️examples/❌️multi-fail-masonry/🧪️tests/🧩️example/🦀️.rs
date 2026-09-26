#[test]
fn noncompliant_example_dsl_asset_parses_and_fails() {
    let text = crate::multi_fail_masonry::PRIMARY_TEXT;
    assert!(text.len() > 40, "DSL asset too short");
    let doc = <crate::En1996Snapshot as store::ArtifactDsl>::parse_dsl(text).expect("parse multi-fail DSL asset");
    assert_eq!(doc.walls[0].id, "wall-weak");
    let report = crate::standards::v1::subsets::any::schema::inferences::evaluate(&doc);
    assert!(!report.complies());
    assert!(report.summary.fail >= 2, "fail_count={}", report.summary.fail);
    for f in report.failing() {
        assert!(!f.remedies.is_empty(), "fail {} missing remedies", f.id);
    }
}
