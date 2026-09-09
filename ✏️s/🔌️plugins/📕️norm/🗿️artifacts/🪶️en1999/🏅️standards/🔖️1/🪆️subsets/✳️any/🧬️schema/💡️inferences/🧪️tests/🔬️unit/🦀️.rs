use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = En1999Snapshot::default();
    assert_eq!(En1999Inference::infer(&snapshot), En1999Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(En1999Inference::infer(&En1999Snapshot::default()), En1999Inference::default());
}
