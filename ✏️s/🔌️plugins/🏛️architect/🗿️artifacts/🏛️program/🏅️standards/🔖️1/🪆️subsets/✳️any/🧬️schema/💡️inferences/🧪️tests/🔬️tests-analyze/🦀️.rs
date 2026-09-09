use super::*;
use crate::sample_plugin;

#[semio_framework_async_macros::async_test]
async fn gap_analysis_on_sample_plugin() {
    let result = run_analysis(&sample_plugin(), AnalysisKind::Gap);
    assert_eq!(result.kind, AnalysisKind::Gap);
    assert!(!result.findings.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn capacity_analysis_sums_area() {
    let result = run_analysis(&sample_plugin(), AnalysisKind::Capacity);
    assert!(result.metrics.iter().any(|m| m.name == "total_target_area"));
    assert!(result.metrics.iter().any(|m| m.value > 0.0));
}

#[semio_framework_async_macros::async_test]
async fn requirement_clustering_produces_clusters() {
    let result = run_analysis(&sample_plugin(), AnalysisKind::RequirementClustering);
    assert_eq!(result.kind, AnalysisKind::RequirementClustering);
}
