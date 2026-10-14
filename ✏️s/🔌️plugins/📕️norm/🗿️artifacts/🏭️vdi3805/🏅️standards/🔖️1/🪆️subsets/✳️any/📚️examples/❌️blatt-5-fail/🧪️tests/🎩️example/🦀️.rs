#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_and_fails() {
    use semio_s_artifact_norm_vdi3805_inferences::evaluate;
    use semio_s_artifact_norm_vdi3805_io::text::snapshot::decode_vdi3805_dsl;
    let text = super::PRIMARY_TEXT;
    let decoded = decode_vdi3805_dsl(text).expect("blatt-5-fail dsl");
    assert_eq!(decoded.catalog.products[0].sheet.0, 5);
    let report = evaluate(&decoded);
    assert!(report.failing().count() >= 2, "blatt-5-fail expected ≥2 fails");
    for check in report.failing() {
        assert!(check.remedies.iter().any(|r| r.applicable), "fail {} needs applicable remedy", check.id);
    }
}
