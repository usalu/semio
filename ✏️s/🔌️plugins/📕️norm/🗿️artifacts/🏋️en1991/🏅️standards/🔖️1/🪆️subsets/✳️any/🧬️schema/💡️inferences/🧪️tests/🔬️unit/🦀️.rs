use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = En1991Snapshot::default();
    assert_eq!(En1991Inference::infer(&snapshot), En1991Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(En1991Inference::infer(&En1991Snapshot::default()), En1991Inference::default());
}
