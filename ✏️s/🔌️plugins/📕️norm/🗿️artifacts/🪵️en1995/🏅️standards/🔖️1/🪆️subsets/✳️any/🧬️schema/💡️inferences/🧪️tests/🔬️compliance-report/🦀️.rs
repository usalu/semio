
use super::*;

#[semio_framework_async_macros::async_test]
async fn full_timber_worked_example() {
    let report = check_full_timber(25.0, 50.0, 15.0, 1_800_000.0, 20_000.0, 200.0, 300.0, 24.0, 21.0, 4.0, ServiceClass::Sc1, LoadDuration::Medium, 80.0, 18.0, 12_000.0, 30.0, 300.0, AnnexChoice::De, 0.3, 500_000.0);
    assert_eq!(report.checks.len(), 8);
    assert!(report.checks[0].utilization < 1.0, "beam bending check should pass");
    assert!(report.checks[2].utilization < 1.0, "shear check should pass");
    assert!(report.checks[5].utilization < 1.0, "bridge bending check should pass");
    assert!(report.checks[6].utilization < 1.0, "pedestrian vibration check should pass");
    assert!(report.checks[7].utilization < 1.0, "bridge fatigue check should pass");
}

#[semio_framework_async_macros::async_test]
async fn evaluate_runs_all_parts() {
    let report = evaluate(&En1995Snapshot::default());
    assert_eq!(report.checks.len(), 8);
}
