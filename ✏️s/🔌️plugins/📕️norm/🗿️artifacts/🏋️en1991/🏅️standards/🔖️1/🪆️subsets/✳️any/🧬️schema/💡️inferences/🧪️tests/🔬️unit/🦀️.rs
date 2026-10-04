use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = En1991Snapshot::default();
    assert_eq!(En1991Inference::infer(&snapshot).expect("valid materialized inference fixture"), En1991Inference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(En1991Inference::infer(&En1991Snapshot::default()).expect("valid materialized inference fixture"), En1991Inference::default());
}
