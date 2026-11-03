use crate as inferences;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[semio_framework_async_macros::async_test]
async fn evaluate_default_runs() {
    let report = inferences::evaluate(&En1998Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
    assert!(!report.checks.is_empty());
}
