use semio_s_plugin_norm_compliance::CheckStatus;
use semio_s_artifact_norm_en1991_inferences::evaluate;
use semio_s_artifact_norm_en1991_io::text::snapshot::decode_en1991_dsl;

#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_and_evaluates() {
    let text = include_str!("../../../../🖼️assets/⚠️multi-fail-noncompliant/⚠️multi-fail-noncompliant/🗣️.dsl.semio");
    let doc = decode_en1991_dsl(text).expect("dsl decodes");
    let report = evaluate(&doc);
    let fails = report.checks.iter().filter(|c| c.status == CheckStatus::Fail).count();
    assert!(fails >= 2, "expected ≥2 fails, got {fails}");
}
