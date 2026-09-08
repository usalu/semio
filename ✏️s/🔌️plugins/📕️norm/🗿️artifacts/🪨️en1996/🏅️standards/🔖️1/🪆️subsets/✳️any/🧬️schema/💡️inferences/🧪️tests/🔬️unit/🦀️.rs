
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = En1996Snapshot::default();
    assert_eq!(En1996Inference::infer(&snapshot), En1996Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(En1996Inference::infer(&En1996Snapshot::default()), En1996Inference::default());
}
