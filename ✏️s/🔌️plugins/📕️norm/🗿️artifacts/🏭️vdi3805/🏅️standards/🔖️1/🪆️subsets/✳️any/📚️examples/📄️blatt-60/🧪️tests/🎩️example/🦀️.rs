#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_and_complies() {
    use crate::standards::v1::subsets::any::schema::inferences::evaluate;
    use crate::standards::v1::subsets::any::schema::snapshot::decode_vdi3805_dsl;
    use crate::conforming_blatt_dataset;
    let text = include_str!("../../../../🖼️assets/blatt-60/🗣️.dsl.semio");
    let decoded = decode_vdi3805_dsl(text).expect("blatt-60 dsl");
    assert_eq!(decoded.catalog.products[0].sheet.0, 60);
    assert!(evaluate(&decoded).complies(), "blatt-60 must comply");
    let expected = conforming_blatt_dataset(60);
    assert_eq!(decoded.catalog.products[0].id, expected.catalog.products[0].id);
}
