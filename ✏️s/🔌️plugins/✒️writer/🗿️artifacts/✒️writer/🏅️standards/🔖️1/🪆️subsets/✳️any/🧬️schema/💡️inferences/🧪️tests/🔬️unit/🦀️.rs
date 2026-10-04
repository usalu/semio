use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = WriterSnapshot::default();
    assert_eq!(WriterInference::infer(&snapshot).expect("valid materialized inference fixture"), WriterInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(WriterInference::infer(&WriterSnapshot::default()).expect("valid materialized inference fixture"), WriterInference::default());
}
