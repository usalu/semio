#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🖼️assets/🎬️demo/🗣️.dsl.semio")).expect("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio").leak();
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
use semio_s_artifact_process_process3d_inferences::Process3dInference;
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = Process3dSnapshot::default();
    assert_eq!(Process3dInference::infer(&snapshot).expect("valid materialized inference fixture"), Process3dInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Process3dInference::infer(&Process3dSnapshot::default()).expect("valid materialized inference fixture"), Process3dInference::default());
}
//#endregion 🧪️InferenceLaws
