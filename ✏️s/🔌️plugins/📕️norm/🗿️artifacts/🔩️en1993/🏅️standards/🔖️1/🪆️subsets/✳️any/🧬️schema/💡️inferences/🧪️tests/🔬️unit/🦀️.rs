use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = En1993Snapshot::default();
    assert_eq!(En1993Inference::infer(&snapshot), En1993Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(En1993Inference::infer(&En1993Snapshot::default()), En1993Inference::default());
}
