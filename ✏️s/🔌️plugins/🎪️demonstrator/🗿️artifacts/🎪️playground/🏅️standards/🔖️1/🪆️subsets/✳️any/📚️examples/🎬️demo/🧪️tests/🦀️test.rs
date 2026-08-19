#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::playground::standards::v1::subsets::any::schema::inferences::PlaygroundInference;
    use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
    use protocol::Inference;

    let snapshot = PlaygroundSnapshot::default();
    assert_eq!(PlaygroundInference::infer(&snapshot), PlaygroundInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::playground::standards::v1::subsets::any::schema::inferences::PlaygroundInference;
    use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
    use protocol::Inference;

    assert_eq!(PlaygroundInference::infer(&PlaygroundSnapshot::default()), PlaygroundInference::default());
}
//#endregion 🧪️InferenceLaws
