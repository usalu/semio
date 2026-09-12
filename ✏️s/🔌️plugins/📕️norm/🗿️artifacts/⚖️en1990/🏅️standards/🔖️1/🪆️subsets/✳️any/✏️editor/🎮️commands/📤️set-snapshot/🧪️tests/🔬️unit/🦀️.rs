use super::*;
use crate::op::En1990Mutation;
use semio_framework_plugin::HistoryView;

#[semio_framework_async_macros::async_test]
async fn handle_commits_the_payload_document_under_its_action_id() {
    let projection = En1990Snapshot::default();
    let config = NoConfig::default();
    let text = crate::document::escape_op_text_field(&<En1990Snapshot as store::ArtifactDsl>::print_dsl(&En1990Snapshot::default()));
    let emit = handle(&ReplaceSnapshot { text }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config, window: None }).expect("handle");
    assert_eq!(emit.artifact_mutations, En1990Mutation::from_snapshot(&En1990Snapshot::default(), &En1990Snapshot::default()));
    assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
    assert!(emit.config_mutations.is_empty());
}
