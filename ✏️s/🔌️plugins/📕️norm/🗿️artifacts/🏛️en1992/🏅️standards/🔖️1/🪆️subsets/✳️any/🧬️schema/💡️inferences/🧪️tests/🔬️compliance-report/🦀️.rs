use super::*;
use crate::document::CheckStatus;

#[semio_framework_async_macros::async_test]
#[cfg(feature = "cross-fem")]
fn evaluate_fem_path() {
    let doc = En1992Snapshot { use_fem: true, ..En1992Snapshot::default() };
    let report = evaluate(&doc);
    assert!(!report.checks.is_empty());
    let m_ed = report.checks[0].computed.value / 1_000_000.0;
    assert!((m_ed - 90.0).abs() < 1.0);
}

#[semio_framework_async_macros::async_test]
async fn evaluate_analytical_with_prestress() {
    let doc = En1992Snapshot { p_kn: 800.0, ..En1992Snapshot::default() };
    let report = evaluate(&doc);
    assert_eq!(report.checks.len(), 10);
    assert!(report.checks.iter().all(|c| c.status != CheckStatus::NotApplicable));
}

#[semio_framework_async_macros::async_test]
async fn evaluate_covers_all_parts() {
    let report = evaluate(&En1992Snapshot::default());
    assert_eq!(report.checks.len(), 9);
    let families: Vec<&str> = report.checks.iter().map(|c| c.clause.family.as_str()).collect();
    assert!(families.contains(&"EN 1992-1-1"));
    assert!(families.contains(&"EN 1992-1-2"));
    assert!(families.contains(&"EN 1992-2"));
    assert!(families.contains(&"EN 1992-3"));
    assert!(families.contains(&"EN 1992-4"));
}
