use semio_s_artifact_norm_en1991_inferences::evaluate;
use semio_s_artifact_norm_en1991_io::text::snapshot::decode_en1991_dsl;

#[test]
fn primary_asset_complies_with_no_failing_check() {
    let report = evaluate(&decode_en1991_dsl(crate::de_office_compliant::PRIMARY_TEXT).expect("de-office-compliant DSL decodes"));
    assert!(!report.checks.is_empty());
    assert!(report.failing().next().is_none(), "fails: {:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
}
