
use super::*;

#[semio_framework_async_macros::async_test]
async fn full_composite_worked_example() {
    let report = check_full_composite(180.0, 110.0, 80.0, 250.0, 0.75, 150.0, 20.0, "r60", "trapezoidal", 55.0, "stud_welded", AnnexChoice::De, 19.0, 95.0, 30.0, 450.0, 33_000.0, 40.0, 8.0, 355.0, 2_000_000.0, 40.0);
    assert_eq!(report.checks.len(), 7);
    let m_rd = part_1_1::plastic_moment_partial_knm(80.0, 250.0, 0.75);
    assert!((m_rd - 207.5).abs() < 0.1);
    assert!(report.checks[0].utilization < 1.0);
    assert!(report.checks[2].utilization < 1.0, "stud resistance check should pass");
    assert!(report.checks[3].utilization < 1.0, "shear connection degree check should pass");
    assert!(report.checks[4].utilization < 1.0, "fire check should pass");
    assert!(report.checks[6].utilization < 1.0, "stud fatigue check should pass");
}

#[semio_framework_async_macros::async_test]
async fn evaluate_runs_all_parts() {
    let report = evaluate(&En1994Snapshot::default());
    assert_eq!(report.checks.len(), 7);
}
