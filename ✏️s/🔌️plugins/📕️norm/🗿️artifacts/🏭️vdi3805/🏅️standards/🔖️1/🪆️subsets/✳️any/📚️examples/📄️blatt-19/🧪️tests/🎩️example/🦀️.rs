#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_and_complies() {
    use semio_s_artifact_norm_vdi3805_inferences::evaluate;
    use semio_s_artifact_norm_vdi3805_io::text::snapshot::decode_vdi3805_dsl;
    use crate::conforming_blatt_dataset;
    let text = semio_s_artifact_norm_vdi3805_examples::blatt_19::PRIMARY_TEXT;
    let decoded = decode_vdi3805_dsl(text).expect("blatt-19 dsl");
    assert_eq!(decoded.catalog.products[0].sheet.0, 19);
    assert!(evaluate(&decoded).complies(), "blatt-19 must comply");
    let expected = conforming_blatt_dataset(19);
    assert_eq!(decoded.catalog.products[0].id, expected.catalog.products[0].id);
}
