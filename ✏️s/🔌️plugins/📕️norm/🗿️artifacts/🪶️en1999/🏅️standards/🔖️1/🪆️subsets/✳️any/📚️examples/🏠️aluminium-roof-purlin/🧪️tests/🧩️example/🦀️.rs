use semio_s_artifact_norm_en1999_inferences::evaluate;
use semio_s_artifact_norm_en1999_io::text::snapshot::decode_en1999_dsl;

#[test]
fn primary_asset_complies_with_no_failing_check() {
    let report = evaluate(&decode_en1999_dsl(crate::aluminium_roof_purlin::PRIMARY_TEXT).expect("aluminium-roof-purlin DSL decodes"));
    assert!(!report.checks.is_empty());
    assert!(report.failing().next().is_none(), "fails: {:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
}
