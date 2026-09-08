
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = Din16798Snapshot::default();
    assert_eq!(Din16798Inference::infer(&snapshot), Din16798Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Din16798Inference::infer(&Din16798Snapshot::default()), Din16798Inference::default());
}
