use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = En1997Snapshot::default();
    assert_eq!(En1997Inference::infer(&snapshot), En1997Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(En1997Inference::infer(&En1997Snapshot::default()), En1997Inference::default());
}
