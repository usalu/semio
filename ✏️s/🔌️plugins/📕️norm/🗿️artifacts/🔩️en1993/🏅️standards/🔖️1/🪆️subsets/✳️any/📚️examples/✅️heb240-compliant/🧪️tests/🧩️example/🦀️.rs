use semio_s_artifact_norm_en1993_inferences::evaluate;
use semio_s_artifact_norm_en1993_io::text::snapshot::decode_en1993_dsl;

#[test]
fn decoded_example_has_no_failing_check() {
    let snapshot = decode_en1993_dsl(crate::heb240_compliant::PRIMARY_TEXT).expect("heb240-compliant DSL decodes");
    let report = evaluate(&snapshot);
    assert!(report.failing().next().is_none(), "fails: {:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
    assert!(report.checks.iter().any(|c| c.status == semio_s_plugin_norm_compliance::CheckStatus::Pass), "at least one check must pass");
}
