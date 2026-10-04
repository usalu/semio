use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = Din18599Snapshot::default();
    assert_eq!(Din18599Inference::infer(&snapshot).expect("valid materialized inference fixture"), Din18599Inference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Din18599Inference::infer(&Din18599Snapshot::default()).expect("valid materialized inference fixture"), Din18599Inference::default());
}
