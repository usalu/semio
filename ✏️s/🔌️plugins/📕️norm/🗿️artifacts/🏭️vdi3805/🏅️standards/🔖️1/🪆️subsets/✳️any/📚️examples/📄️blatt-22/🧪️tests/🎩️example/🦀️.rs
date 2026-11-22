#[semio_framework_async_macros::async_test]
async fn app_example_decodes_and_checks_part22_records() {
    use semio_s_artifact_norm_vdi3805_inferences::evaluate;
    use semio_s_artifact_norm_vdi3805_io::text::snapshot::decode_vdi3805_dsl;
    use semio_s_plugin_norm_compliance::CheckStatus;

    let text = super::PRIMARY_TEXT;
    let document = decode_vdi3805_dsl(text).expect("Blatt 22 app example");
    assert_eq!(document.catalog.products[0].sheet.0, 22);
    let report = evaluate(&document);
    for code in ["71015", "71016", "71017"] {
        let prefix = format!("vdi3805.22.record{code}.");
        assert!(report.checks.iter().any(|check| check.id.starts_with(&prefix) && check.status == CheckStatus::Pass));
    }
    assert!(report.unverified().any(|check| check.id.starts_with("vdi3805.22.recordSchema.")));
}
