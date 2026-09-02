#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    let snapshot = <crate::artifacts::gismap::GisMapSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::artifacts::gismap::standards::v1::subsets::any::schema::inferences::GisMapInference::infer(&snapshot);
    assert_eq!(inference, crate::artifacts::gismap::standards::v1::subsets::any::schema::inferences::GisMapInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(
        crate::artifacts::gismap::standards::v1::subsets::any::schema::inferences::GisMapInference::infer(&crate::artifacts::gismap::GisMapSnapshot::default()),
        crate::artifacts::gismap::standards::v1::subsets::any::schema::inferences::GisMapInference::default(),
    );
}
//#endregion 🧪️InferenceLaws
