use semio_s_artifact_norm_en1997_inferences::evaluate;
use semio_s_artifact_norm_en1997_io::text::snapshot::decode_en1997_dsl;

#[test]
fn primary_asset_complies_with_no_failing_check() {
    let snapshot = decode_en1997_dsl(crate::demo::PRIMARY_TEXT).expect("demo DSL decodes");
    let report = evaluate(&snapshot);
    assert!(!report.checks.is_empty());
    assert!(report.failing().next().is_none(), "fails: {:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
    let pile = report.checks.iter().find(|c| c.id == "en1997.gi-01.pile.pile-P1").expect("pile investigation depth check present");
    assert_eq!(pile.status, semio_s_plugin_norm_compliance::CheckStatus::Pass);
}
