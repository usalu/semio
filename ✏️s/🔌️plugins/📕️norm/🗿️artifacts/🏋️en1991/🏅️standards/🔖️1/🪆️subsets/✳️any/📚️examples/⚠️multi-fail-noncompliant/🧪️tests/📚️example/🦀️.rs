use crate::document::CheckStatus;
use crate::artifact_schema::inferences::evaluate;
use crate::standards::v1::subsets::any::schema::snapshot::decode_en1991_dsl;

#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_and_evaluates() {
    let text = include_str!("../../../../🖼️assets/⚠️multi-fail-noncompliant/⚠️multi-fail-noncompliant/🗣️.dsl.semio");
    let doc = decode_en1991_dsl(text).expect("dsl decodes");
    let report = evaluate(&doc);
    let fails = report.checks.iter().filter(|c| c.status == CheckStatus::Fail).count();
    assert!(fails >= 2, "expected ≥2 fails, got {fails}");
}
