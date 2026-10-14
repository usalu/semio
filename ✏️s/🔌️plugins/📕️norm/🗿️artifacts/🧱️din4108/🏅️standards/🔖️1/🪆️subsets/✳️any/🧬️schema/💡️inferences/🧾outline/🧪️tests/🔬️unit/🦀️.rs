use crate::outline::Din4108Outline;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

#[semio_framework_async_macros::async_test]
async fn outline_counts_envelope_entities() {
    let o = Din4108Outline::compute(&Din4108Snapshot::default());
    assert_eq!(o.field_count, 10);
    assert!(o.entry_count >= 3);
}
