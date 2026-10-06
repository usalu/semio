use super::*;
use crate::standards::v1::subsets::any::schema::mutations::Vdi3805Mutation;
use semio_framework_plugin::HistoryView;

#[semio_framework_async_macros::async_test]
async fn handle_commits_the_payload_document_as_its_field_mutations() {
    let projection = Vdi3805Snapshot::default();
    let config = NoConfig::default();
    let emit = handle(&ReplaceSnapshot { snapshot: Vdi3805Snapshot::default() }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config, window: None }).expect("handle");
    assert_eq!(emit.artifact_mutations, Vdi3805Mutation::from_snapshot(&Vdi3805Snapshot::default(), &Vdi3805Snapshot::default()));
    assert!(emit.config_mutations.is_empty());
}
