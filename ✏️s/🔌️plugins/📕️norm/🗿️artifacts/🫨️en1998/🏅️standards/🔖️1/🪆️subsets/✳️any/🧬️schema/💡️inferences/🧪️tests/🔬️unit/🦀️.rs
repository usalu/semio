use crate::standards::v1::subsets::any::schema::inferences;
use crate::En1998Snapshot;

#[semio_framework_async_macros::async_test]
async fn evaluate_default_runs() {
    let report = inferences::evaluate(&En1998Snapshot::default());
    assert!(!report.checks.is_empty());
}
