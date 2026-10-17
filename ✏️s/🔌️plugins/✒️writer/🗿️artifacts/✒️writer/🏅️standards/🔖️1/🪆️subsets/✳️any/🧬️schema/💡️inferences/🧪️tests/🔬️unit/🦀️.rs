use crate::WriterSnapshot;
#[allow(unused_imports)]
use framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use crate::standards::v1::subsets::any::schema::inferences::*;
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
