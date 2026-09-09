use super::*;
use crate::sample_plugin;

#[semio_framework_async_macros::async_test]
async fn sample_plugin_status_summary_counts_elements() {
    let summary = status_summary(&sample_plugin());
    assert!(summary.total_entities >= 2);
    let elements = summary.by_register.iter().find(|r| r.register == "elements").expect("elements");
    assert_eq!(elements.count, 2);
}

#[semio_framework_async_macros::async_test]
async fn status_summary_includes_all_major_registers() {
    let summary = status_summary(&sample_plugin());
    for register in ["elements", "stakeholders", "adjacencies", "status_records"] {
        assert!(summary.by_register.iter().any(|r| r.register == register));
    }
}
