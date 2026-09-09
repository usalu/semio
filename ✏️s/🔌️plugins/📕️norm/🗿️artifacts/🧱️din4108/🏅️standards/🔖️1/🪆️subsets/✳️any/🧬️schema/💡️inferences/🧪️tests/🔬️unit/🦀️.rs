use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = Din4108Snapshot::default();
    assert_eq!(Din4108Inference::infer(&snapshot), Din4108Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Din4108Inference::infer(&Din4108Snapshot::default()), Din4108Inference::default());
}
