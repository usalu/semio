use super::*;

#[semio_framework_async_macros::async_test]
async fn residential_environment_e2e_with_acoustic() {
    let report = check_residential_environment(85.0, 3, 40.0, 21.0, 24.0);
    assert!(report.all_pass());
    assert_eq!(report.checks.len(), 3);
}

#[semio_framework_async_macros::async_test]
async fn full_environment_evaluate_covers_all_nine_parts() {
    let document = Din16798Snapshot::default();
    let report = evaluate(&document);
    assert_eq!(report.checks.len(), 25, "checks: {:?}", report.checks);
    assert!(report.all_pass(), "checks: {:?}", report.checks);
    assert_eq!(document.annex, crate::document::AnnexChoice::De);
    let pmv = part_1::pmv_iso7730(document.t_op_c, document.rh_percent, document.air_speed_m_s);
    assert!(pmv.abs() < 0.5);
}
