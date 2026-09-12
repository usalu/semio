use super::*;
use semio_framework_plugin::HistoryView;

#[semio_framework_async_macros::async_test]
async fn handle_emits_only_a_config_operation() {
    let projection = Din16798Snapshot::default();
    let config = NoConfig::default();
    let emit = handle(&SetSelectedCheckIndex { index: Some(4) }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config, window: None }).expect("handle");
    assert!(emit.artifact_mutations.is_empty(), "a view action must never emit document operations");
    assert!(emit.config_mutations.is_empty());
    assert!(emit.window_config_mutations.is_empty());
    assert_eq!(window_mutation(&SetSelectedCheckIndex { index: Some(4) }), crate::results_window_config::ChangeSelectedCheckIndex { index: Some(4) }.into());
}
