#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_and_complies() {
    use crate::standards::v1::subsets::any::schema::check_full_steel_structure;
    use crate::En1993Snapshot;
    use store::ArtifactDsl;
    let text = include_str!("../../../../🖼️assets/✅️heb240-compliant/✅️heb240-compliant/🗣️.dsl.semio");
    let snapshot = En1993Snapshot::parse_dsl(text).expect("decode compliant DSL");
    let report = check_full_steel_structure(&snapshot);
    assert!(report.complies(), "compliant example must Pass");
}
