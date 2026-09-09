use super::*;

#[semio_framework_async_macros::async_test]
async fn full_geotechnical_worked_example() {
    let report = check_full_geotechnical(500.0, 80.0, 2.0, 30.0, 0.0, 18.0, 2.0, 1.5, 30_000.0, 0.3, DesignApproach::Da1Str, AnnexChoice::De, 25.0, 800.0, 0.7, 0.6, 80.0, 12.0, 2500.0, 0.28, 1, 8.0);
    assert_eq!(report.checks.len(), 5);
    assert!(report.checks[3].utilization < 1.0);
    assert_eq!(report.checks[4].status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn evaluate_runs_all_parts() {
    let report = evaluate(&En1997Snapshot::default());
    assert_eq!(report.checks.len(), 5);
}
