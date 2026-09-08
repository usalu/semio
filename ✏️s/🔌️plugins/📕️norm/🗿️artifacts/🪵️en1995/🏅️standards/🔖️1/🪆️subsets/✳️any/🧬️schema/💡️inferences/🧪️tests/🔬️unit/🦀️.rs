
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = En1995Snapshot::default();
    assert_eq!(En1995Inference::infer(&snapshot), En1995Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(En1995Inference::infer(&En1995Snapshot::default()), En1995Inference::default());
}
