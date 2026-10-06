use super::*;
use crate::standards::v1::subsets::any::schema::mutations::En1997Mutation;
use semio_framework_plugin::HistoryView;

#[semio_framework_async_macros::async_test]
async fn handle_commits_the_payload_document_as_its_field_mutations() {
    let projection = En1997Snapshot::default();
    let config = NoConfig::default();
    let emit = handle(&ReplaceSnapshot { snapshot: En1997Snapshot::default() }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config, window: None }).expect("handle");
    assert_eq!(emit.artifact_mutations, En1997Mutation::from_snapshot(&En1997Snapshot::default(), &En1997Snapshot::default()));
    assert!(emit.config_mutations.is_empty());
}
