use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = Iso16757Snapshot::default();
    assert_eq!(Iso16757Inference::infer(&snapshot).expect("valid materialized inference fixture"), Iso16757Inference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Iso16757Inference::infer(&Iso16757Snapshot::default()).expect("valid materialized inference fixture"), Iso16757Inference::default());
}
