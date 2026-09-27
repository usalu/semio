#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_and_fails() {
    use crate::standards::v1::subsets::any::schema::inferences::evaluate;
    use crate::standards::v1::subsets::any::schema::snapshot::decode_vdi3805_dsl;
    let text = include_str!("../../../../🖼️assets/blatt-53-fail/🗣️.dsl.semio");
    let decoded = decode_vdi3805_dsl(text).expect("blatt-53-fail dsl");
    assert_eq!(decoded.catalog.products[0].sheet.0, 53);
    let report = evaluate(&decoded);
    assert!(report.failing().count() >= 2, "blatt-53-fail expected ≥2 fails");
    for check in report.failing() {
        assert!(check.remedies.iter().any(|r| r.applicable), "fail {} needs applicable remedy", check.id);
    }
}
