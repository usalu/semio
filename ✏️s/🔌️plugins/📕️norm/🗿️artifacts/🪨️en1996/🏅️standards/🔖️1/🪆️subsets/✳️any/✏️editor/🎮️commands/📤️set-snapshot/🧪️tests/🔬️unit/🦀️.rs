use super::*;
use crate::op::En1996Mutation;
use semio_framework_plugin::HistoryView;

#[semio_framework_async_macros::async_test]
async fn handle_commits_the_payload_document_under_its_action_id() {
    let projection = En1996Snapshot::default();
    let config = NoConfig::default();
    let emit = handle(&ReplaceSnapshot { snapshot: En1996Snapshot::default() }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config, window: None }).expect("handle");
    assert_eq!(emit.artifact_mutations, En1996Mutation::from_snapshot(&En1996Snapshot::default()));
    assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
    assert!(emit.config_mutations.is_empty());
}
