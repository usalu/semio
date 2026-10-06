use super::*;
use crate::standards::v1::subsets::any::schema::mutations::Din18599Mutation;
use semio_framework_plugin::HistoryView;

#[semio_framework_async_macros::async_test]
async fn handle_commits_the_payload_document_as_its_field_mutations() {
    let projection = Din18599Snapshot::default();
    let config = NoConfig::default();
    let text = crate::document::escape_op_text_field(&<Din18599Snapshot as store::ArtifactDsl>::print_dsl(&Din18599Snapshot::default()));
    let emit = handle(&ReplaceSnapshot { text }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config, window: None }).expect("handle");
    assert_eq!(emit.artifact_mutations, Din18599Mutation::from_snapshot(&projection, &Din18599Snapshot::default()));
    assert!(emit.config_mutations.is_empty());
}
