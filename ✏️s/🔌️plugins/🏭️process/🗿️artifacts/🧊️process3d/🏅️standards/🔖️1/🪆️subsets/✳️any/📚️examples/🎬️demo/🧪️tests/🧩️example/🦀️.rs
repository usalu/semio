#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
use crate::standards::v1::subsets::any::schema::inferences::Process3dInference;
use crate::Process3dSnapshot;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = Process3dSnapshot::default();
    assert_eq!(Process3dInference::infer(&snapshot), Process3dInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Process3dInference::infer(&Process3dSnapshot::default()), Process3dInference::default());
}
//#endregion 🧪️InferenceLaws
