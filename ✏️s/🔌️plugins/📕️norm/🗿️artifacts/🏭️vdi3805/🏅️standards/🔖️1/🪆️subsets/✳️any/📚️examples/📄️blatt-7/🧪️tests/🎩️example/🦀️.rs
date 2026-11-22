#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_and_complies() {
    use semio_s_artifact_norm_vdi3805_inferences::evaluate;
    use semio_s_artifact_norm_vdi3805_io::text::snapshot::decode_vdi3805_dsl;
    use semio_s_artifact_norm_vdi3805_model::conforming_blatt_dataset;
    let text = super::PRIMARY_TEXT;
    let decoded = decode_vdi3805_dsl(text).expect("blatt-7 dsl");
    assert_eq!(decoded.catalog.products[0].sheet.0, 7);
    let report = evaluate(&decoded);
    assert!(report.failing().next().is_none());
    assert!(report.unverified().any(|check| check.id == "vdi3805.7.recordSchema.FAN-01"));
    let expected = conforming_blatt_dataset(7);
    assert_eq!(decoded.catalog.products[0].id, expected.catalog.products[0].id);
}
