use crate::commands::selected_check::*;
use semio_plugin::HistoryView;
use semio_plugin::NoConfig;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;

#[semio_framework_async_macros::async_test]
async fn handle_emits_only_a_config_operation() {
    let projection = Vdi3805Snapshot::default();
    let config = NoConfig::default();
    let emit = handle(&SetSelectedCheckIndex { index: Some(4) }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config, window: None }).expect("handle");
    assert!(emit.artifact_mutations.is_empty(), "a view action must never emit document operations");
    assert!(emit.config_mutations.is_empty());
    assert!(emit.window_config_mutations.is_empty());
    assert_eq!(window_mutation(&SetSelectedCheckIndex { index: Some(4) }), semio_s_plugin_norm_results_config::ChangeSelectedCheckIndex { index: Some(4) }.into());
}
