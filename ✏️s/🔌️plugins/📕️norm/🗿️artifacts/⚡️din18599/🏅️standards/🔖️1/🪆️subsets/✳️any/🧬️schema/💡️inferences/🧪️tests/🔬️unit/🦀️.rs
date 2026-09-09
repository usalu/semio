use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = Din18599Snapshot::default();
    assert_eq!(Din18599Inference::infer(&snapshot), Din18599Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Din18599Inference::infer(&Din18599Snapshot::default()), Din18599Inference::default());
}
