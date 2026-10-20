#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_and_complies() {
    use semio_s_artifact_norm_vdi3805_inferences::evaluate;
    use semio_s_artifact_norm_vdi3805_io::text::snapshot::decode_vdi3805_dsl;
    use semio_s_artifact_norm_vdi3805_model::conforming_blatt_dataset;
    let text = semio_s_artifact_norm_vdi3805_examples::blatt_8::PRIMARY_TEXT;
    let decoded = decode_vdi3805_dsl(text).expect("blatt-8 dsl");
    assert_eq!(decoded.catalog.products[0].sheet.0, 8);
    assert!(evaluate(&decoded).complies(), "blatt-8 must comply");
    let expected = conforming_blatt_dataset(8);
    assert_eq!(decoded.catalog.products[0].id, expected.catalog.products[0].id);
}
