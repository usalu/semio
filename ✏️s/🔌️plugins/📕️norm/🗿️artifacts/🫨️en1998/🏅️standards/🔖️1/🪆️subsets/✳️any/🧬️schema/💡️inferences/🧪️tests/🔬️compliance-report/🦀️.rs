use super::*;
use crate::document::CheckStatus;
#[semio_framework_async_macros::async_test]
async fn default_evaluate_complies() {
    let report = evaluate(&crate::En1998Snapshot::default());
    assert!(report.complies());
}
#[semio_framework_async_macros::async_test]
async fn empty_scopes_are_not_applicable() {
    let mut doc = crate::En1998Snapshot::compliant_de_office();
    doc.bridges.clear();
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id == "en1998.2.na" && c.status == CheckStatus::NotApplicable));
}
