use semio_s_artifact_norm_en1998_inferences::evaluate;
use semio_s_artifact_norm_en1998_io::text::snapshot::decode_en1998_dsl;

#[test]
fn decoded_example_has_no_failing_check() {
    let snapshot = decode_en1998_dsl(crate::seismic_multipart::PRIMARY_TEXT).expect("seismic-multipart DSL decodes");
    let report = evaluate(&snapshot);
    assert!(report.failing().next().is_none(), "fails: {:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
    assert!(report.checks.iter().any(|c| c.status == semio_s_plugin_norm_compliance::CheckStatus::Pass), "at least one check must pass");
}
