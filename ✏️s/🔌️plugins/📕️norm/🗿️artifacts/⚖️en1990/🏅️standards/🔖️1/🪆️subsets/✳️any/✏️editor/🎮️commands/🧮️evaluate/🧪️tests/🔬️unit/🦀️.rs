
use super::*;
use semio_framework_plugin::HistoryView;

#[semio_framework_async_macros::async_test]
async fn handle_emits_no_mutation_since_the_report_is_always_recomputed() {
    let projection = En1990Snapshot::default();
    let config = NormConfig::default();
    let emit = handle(&Evaluate {}, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config }).expect("handle");
    assert!(emit.artifact_mutations.is_empty());
    assert!(emit.config_mutations.is_empty());
}
