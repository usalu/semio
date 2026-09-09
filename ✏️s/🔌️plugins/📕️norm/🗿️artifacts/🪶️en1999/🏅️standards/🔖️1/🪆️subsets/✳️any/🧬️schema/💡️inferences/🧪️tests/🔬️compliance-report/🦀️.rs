use super::*;

#[semio_framework_async_macros::async_test]
async fn full_aluminium_worked_example() {
    let alloy = part_1_1::Alloy::Aw6060T6;
    let report = check_full_aluminium(80.0, 4.0, 1200.0, 24_000.0, alloy, 0.85, 5000.0, 3000.0, 200.0, 45.0, 71.0, 8.0, 500_000.0, 25.0, 4.0, 120.0, 0.63, 200.0, 2.0, 4.0, 8000.0, 0.5, 4.0, 500.0, 150.0, AnnexChoice::De);
    assert_eq!(report.checks.len(), 8);
    assert!(report.checks[4].utilization < 1.0);
}

#[semio_framework_async_macros::async_test]
async fn evaluate_runs_all_parts() {
    let report = evaluate(&En1999Snapshot::default());
    assert_eq!(report.checks.len(), 8);
}

#[semio_framework_async_macros::async_test]
async fn annex_en_de_documented_equality() {
    // 📖️ DIN EN 1999-1-1/NA does not override γ_M1/γ_M2, so EN and DE-NA must yield identical utilization.
    let en_doc = En1999Snapshot { annex: AnnexChoice::En, ..En1999Snapshot::default() };
    let de_doc = En1999Snapshot { annex: AnnexChoice::De, ..En1999Snapshot::default() };
    let en_report = evaluate(&en_doc);
    let de_report = evaluate(&de_doc);
    assert_eq!(en_report.checks.len(), de_report.checks.len());
    for (en_check, de_check) in en_report.checks.iter().zip(de_report.checks.iter()) {
        assert!((en_check.utilization - de_check.utilization).abs() < 1e-9);
    }
}
