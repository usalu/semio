//! 🔬️ End-to-end compliance report for hierarchical din16798 subjects.

use crate::evaluate;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;

#[test]
fn residential_example_evaluates_unoccupied_airflow_separately() {
    use semio_s_plugin_norm_compliance::{AnnexChoice, CheckStatus};
    let mut document=Din16798Snapshot::residential_method3();
    let status=|snapshot:&Din16798Snapshot|evaluate(snapshot).checks.into_iter()
        .find(|check|check.id=="din16798-1.vent.unoccupied.zone-flat").unwrap().status;
    assert_eq!(status(&document),CheckStatus::Pass);
    document.zones[0].unoccupied_ventilation.as_mut().unwrap().outdoor_air_m3_h=30.0;
    assert_eq!(status(&document),CheckStatus::Fail);
    document.annex=AnnexChoice::De;
    assert_eq!(status(&document),CheckStatus::UnverifiedBasis);
}

#[test]
fn compliant_office_evaluate_complies() {
    let report = evaluate(&Din16798Snapshot::compliant_office());
    assert!(report.complies());
    assert!(report.summary.total > 10);
}

#[test]
fn noncompliant_office_evaluate_fails_across_parts() {
    let report = evaluate(&Din16798Snapshot::noncompliant_office());
    assert!(!report.complies());
    let parts: std::collections::BTreeSet<_> = report.failing().map(|c| c.part.as_str()).collect();
    assert!(parts.contains("DIN EN 16798-1"));
    assert!(parts.iter().any(|p| p.contains("16798-3") || p.contains("16798-5") || p.contains("16798-7") || p.contains("16798-17")), "parts={parts:?}");
}

#[test]
fn empty_zones_marks_na_not_hard_fail_only() {
    let mut doc = Din16798Snapshot::compliant_office();
    doc.zones.clear();
    let report = evaluate(&doc);
    assert!(report.summary.not_applicable >= 1);
}
