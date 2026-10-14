#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🌉️composite-bridge-girder/🌉️composite-bridge-girder/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use semio_s_artifact_norm_en1994_inferences::En1994Inference;
    use semio_s_artifact_norm_en1994_model::En1994Snapshot;
    use protocol::Inference;
    let snapshot = En1994Snapshot::default();
    assert_eq!(En1994Inference::infer(&snapshot).expect("valid materialized inference fixture"), En1994Inference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use semio_s_artifact_norm_en1994_inferences::En1994Inference;
    use semio_s_artifact_norm_en1994_model::En1994Snapshot;
    use protocol::Inference;
    assert_eq!(En1994Inference::infer(&En1994Snapshot::default()).expect("valid materialized inference fixture"), En1994Inference::default());
}
