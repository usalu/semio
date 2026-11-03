use crate::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = En1997Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    assert_eq!(En1997Inference::infer(&snapshot).expect("valid materialized inference fixture"), En1997Inference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(En1997Inference::infer(&En1997Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)).expect("valid materialized inference fixture"), En1997Inference::default());
}
