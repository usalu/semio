use crate::Din4108Snapshot;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_builds_outline() {
    let inf = <crate::standards::v1::subsets::any::schema::inferences::Din4108Inference as Inference<Din4108Snapshot>>::infer(&Din4108Snapshot::default());
    assert!(inf.outline.field_count >= 10);
}
