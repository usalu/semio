#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let snapshot = <crate::artifacts::wires::WiresSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::WiresInference::infer(&snapshot);
    assert_eq!(inference, crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::WiresInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(
        crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::WiresInference::infer(&crate::artifacts::wires::empty_wires_snapshot()),
        crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::WiresInference::default(),
    );
}
//#endregion 🧪️InferenceLaws
