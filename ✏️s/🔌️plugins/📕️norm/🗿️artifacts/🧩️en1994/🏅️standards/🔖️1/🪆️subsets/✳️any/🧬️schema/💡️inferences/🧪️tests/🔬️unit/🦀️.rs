use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = En1994Snapshot::default();
    assert_eq!(En1994Inference::infer(&snapshot).expect("valid materialized inference fixture"), En1994Inference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(En1994Inference::infer(&En1994Snapshot::default()).expect("valid materialized inference fixture"), En1994Inference::default());
}
