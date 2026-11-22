use semio_s_artifact_norm_din16798_inferences::evaluate;
use semio_s_artifact_norm_din16798_io::text::snapshot::decode_din16798_dsl;

#[test]
fn primary_asset_complies_with_no_failing_check() {
    let report = evaluate(&decode_din16798_dsl(crate::residential_method3::PRIMARY_TEXT).expect("residential-method3 DSL decodes"));
    assert!(!report.checks.is_empty());
    assert!(report.failing().next().is_none(), "fails: {:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
    let check = report.checks.iter().find(|c| c.id == "din16798-1.top.summer.zone-flat").expect("din16798-1.top.summer.zone-flat present");
    assert_eq!(check.status, semio_s_plugin_norm_compliance::CheckStatus::Pass);
}
