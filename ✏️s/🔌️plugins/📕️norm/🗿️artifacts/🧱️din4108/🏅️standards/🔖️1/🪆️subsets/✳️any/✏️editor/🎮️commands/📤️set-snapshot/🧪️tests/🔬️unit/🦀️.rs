use super::*;
use crate::op::Din4108Mutation;
use semio_framework_plugin::HistoryView;

#[semio_framework_async_macros::async_test]
async fn handle_commits_the_payload_document_under_its_action_id() {
    let projection = Din4108Snapshot::default();
    let config = NormConfig::default();
    let emit = handle(&ReplaceSnapshot { snapshot: Din4108Snapshot::default() }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config, window: None }).expect("handle");
    assert_eq!(emit.artifact_mutations, Din4108Mutation::from_snapshot(&Din4108Snapshot::default(), &Din4108Snapshot::default()));
    assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
    assert!(emit.config_mutations.is_empty());
}
