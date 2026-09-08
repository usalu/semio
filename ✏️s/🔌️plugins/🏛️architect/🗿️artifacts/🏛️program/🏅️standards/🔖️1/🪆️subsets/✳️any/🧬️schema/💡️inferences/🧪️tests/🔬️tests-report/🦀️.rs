
use super::*;
use crate::sample_plugin;

#[semio_framework_async_macros::async_test]
async fn executive_summary_includes_counts() {
    let report = build_report(&sample_plugin(), ReportKind::ExecutiveSummary);
    assert_eq!(report.kind, ReportKind::ExecutiveSummary);
    assert!(!report.sections.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn requirements_matrix_has_grid_rows() {
    let report = build_report(&sample_plugin(), ReportKind::RequirementsMatrix);
    assert!(!report.sections[0].bullets.is_empty());
    assert!(report.sections[0].bullets[0].contains('\t'));
}
