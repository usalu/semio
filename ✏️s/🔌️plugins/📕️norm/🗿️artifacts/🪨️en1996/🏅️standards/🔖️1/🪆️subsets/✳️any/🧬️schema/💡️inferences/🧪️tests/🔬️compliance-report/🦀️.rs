use super::*;

#[semio_framework_async_macros::async_test]
async fn full_masonry_worked_example() {
    let report = check_full_masonry(&En1996Snapshot::default());
    assert_eq!(report.checks.len(), 8);
}

#[semio_framework_async_macros::async_test]
async fn evaluate_runs_all_parts() {
    let report = evaluate(&En1996Snapshot::default());
    assert_eq!(report.checks.len(), 8);
}
