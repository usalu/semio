#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🏢️road-bridge-failing/🏢️road-bridge-failing/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use semio_s_artifact_norm_en1990_inferences::En1990Inference;
    use semio_s_artifact_norm_en1990_model::En1990Snapshot;
    use protocol::Inference;
    let snapshot = En1990Snapshot::default();
    assert_eq!(En1990Inference::infer(&snapshot).expect("valid materialized inference fixture"), En1990Inference::infer(&snapshot).expect("valid materialized inference fixture"));
}
