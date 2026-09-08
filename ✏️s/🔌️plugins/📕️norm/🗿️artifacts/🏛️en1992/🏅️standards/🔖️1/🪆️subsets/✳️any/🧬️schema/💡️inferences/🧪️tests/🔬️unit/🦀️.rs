
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = En1992Snapshot::default();
    assert_eq!(En1992Inference::infer(&snapshot), En1992Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(En1992Inference::infer(&En1992Snapshot::default()), En1992Inference::default());
}
