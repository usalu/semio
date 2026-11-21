#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_with_unverified_sheet_records() {
    use semio_s_artifact_norm_vdi3805_inferences::evaluate;
    use semio_s_artifact_norm_vdi3805_io::text::snapshot::decode_vdi3805_dsl;
    use semio_s_artifact_norm_vdi3805_model::conforming_blatt_dataset;
    let text = semio_s_artifact_norm_vdi3805_examples::blatt_6::PRIMARY_TEXT;
    let decoded = decode_vdi3805_dsl(text).expect("blatt-6 dsl");
    assert_eq!(decoded.catalog.products[0].sheet.0, 6);
    let report = evaluate(&decoded);
    assert_eq!(report.failing().count(), 0);
    assert!(report.unverified().any(|check| check.id.starts_with("vdi3805.6.recordSchema.")));
    let expected = conforming_blatt_dataset(6);
    assert_eq!(decoded.catalog.products[0].id, expected.catalog.products[0].id);
}
