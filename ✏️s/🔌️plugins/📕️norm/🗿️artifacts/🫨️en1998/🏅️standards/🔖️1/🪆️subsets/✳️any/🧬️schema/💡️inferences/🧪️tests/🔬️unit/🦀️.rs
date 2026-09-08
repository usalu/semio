
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = En1998Snapshot::default();
    assert_eq!(En1998Inference::infer(&snapshot), En1998Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(En1998Inference::infer(&En1998Snapshot::default()), En1998Inference::default());
}
