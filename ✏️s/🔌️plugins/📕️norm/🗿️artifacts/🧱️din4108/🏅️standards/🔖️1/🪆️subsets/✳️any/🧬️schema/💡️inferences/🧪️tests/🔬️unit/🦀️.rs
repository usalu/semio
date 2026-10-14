use semio_s_artifact_norm_din4108_model::Din4108Snapshot;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_builds_outline() {
    let inf = <crate::Din4108Inference as Inference<Din4108Snapshot>>::infer(&Din4108Snapshot::default()).expect("valid materialized inference fixture");
    assert!(inf.outline.field_count >= 10);
}
