
use super::*;
use semio_framework_plugin::HistoryView;

#[semio_framework_async_macros::async_test]
async fn handle_emits_only_a_config_operation() {
    let projection = En1994Snapshot::default();
    let config = NormConfig::default();
    let emit = handle(&SetSelectedCheckIndex { index: Some(4) }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config }).expect("handle");
    assert!(emit.artifact_mutations.is_empty(), "a view action must never emit document operations");
    assert_eq!(emit.config_mutations, vec![NormConfigMutation::ChangeSelectedCheckIndex(crate::config::ChangeSelectedCheckIndex { index: Some(4) })]);
}
