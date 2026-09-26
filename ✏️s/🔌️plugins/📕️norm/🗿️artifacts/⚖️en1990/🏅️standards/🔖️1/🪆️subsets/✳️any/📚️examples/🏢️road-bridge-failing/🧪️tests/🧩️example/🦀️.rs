#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🏢️road-bridge-failing/🏢️road-bridge-failing/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifact_schema::inferences::En1990Inference;
    use crate::En1990Snapshot;
    use protocol::Inference;
    let snapshot = En1990Snapshot::default();
    assert_eq!(En1990Inference::infer(&snapshot), En1990Inference::infer(&snapshot));
}
