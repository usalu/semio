use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = En1992Snapshot::default();
    assert_eq!(En1992Inference::infer(&snapshot).expect("valid materialized inference fixture"), En1992Inference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(En1992Inference::infer(&En1992Snapshot::default()).expect("valid materialized inference fixture"), En1992Inference::default());
}
