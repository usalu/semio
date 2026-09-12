use super::*;
use crate::op::Din18599Mutation;
use semio_framework_plugin::HistoryView;

#[semio_framework_async_macros::async_test]
async fn handle_commits_the_payload_document_under_its_action_id() {
    let projection = Din18599Snapshot::default();
    let config = NoConfig::default();
    let text = crate::document::escape_op_text_field(&<Din18599Snapshot as store::ArtifactDsl>::print_dsl(&Din18599Snapshot::default()));
    let emit = handle(&ReplaceSnapshot { text }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config, window: None }).expect("handle");
    assert_eq!(emit.artifact_mutations, Din18599Mutation::from_snapshot(&Din18599Snapshot::default()));
    assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
    assert!(emit.config_mutations.is_empty());
}

/// 🌡️ Regression guard: this command's whole reason for using `ArtifactDsl` text instead of
/// `serde_json` — a value whose shortest round-trip representation needs its full 17
/// significant digits must survive the command's own payload encoding exactly.
#[semio_framework_async_macros::async_test]
async fn handle_preserves_full_f64_precision_through_the_payload() {
    let projection = Din18599Snapshot::default();
    let config = NoConfig::default();
    let text = crate::document::escape_op_text_field(&<Din18599Snapshot as store::ArtifactDsl>::print_dsl(&Din18599Snapshot::default()));
    let emit = handle(&ReplaceSnapshot { text }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config, window: None }).expect("handle");
    let restored = emit.artifact_mutations.iter().fold(Din18599Snapshot::default(), |snapshot, mutation| vcs::apply_mutation(&snapshot, mutation).expect("set-snapshot mutation applies").0);
    assert_eq!(restored.h_v, Din18599Snapshot::default().h_v, "h_v must survive the set-snapshot payload with full precision");
}
