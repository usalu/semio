
use super::*;
use crate::op::Din16798Mutation;
use semio_framework_plugin::HistoryView;

#[semio_framework_async_macros::async_test]
async fn handle_commits_the_payload_document_under_its_action_id() {
    let projection = Din16798Snapshot::default();
    let config = NormConfig::default();
    let emit = handle(&ReplaceSnapshot { snapshot: Din16798Snapshot::default() }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config }).expect("handle");
    assert_eq!(emit.artifact_mutations, Din16798Mutation::from_snapshot(&Din16798Snapshot::default()));
    assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
    assert!(emit.config_mutations.is_empty());
}
