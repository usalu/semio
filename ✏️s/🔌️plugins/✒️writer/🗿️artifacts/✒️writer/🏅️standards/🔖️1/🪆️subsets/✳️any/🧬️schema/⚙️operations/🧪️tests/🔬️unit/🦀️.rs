use crate::schema::mutations::WriterMutation;
use crate::schema::mutations::{ChangeLanguage, ChangeUri, EditText, RenameWriter};
use crate::WriterSnapshot;
#[allow(unused_imports)]
use protocol::{ColdMutation, Mutation};
use crate::standards::v1::subsets::any::schema::operations::*;
use crate::schema;

/// 🔐️ A store WITH its owner catalog installed. `ArtifactStore::new` installs none, and
/// `reserve_edit_history_slot` then refuses every `Apply` with
/// `edit history insertion requires its exact mutation retirement factory` — the fixture could be
/// read but never mutated or undone (fleet-brief stale-test bucket 2).
async fn seeded_store() -> crate::host::owned::OwnedWriterStore {
    crate::host::owned::new_writer_store(store::os_store::document_materialize::create_document_envelope(crate::WRITER_DOCUMENT_SCHEMA, "writer", schema::empty_writer_snapshot(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture")
}

#[semio_framework_async_macros::async_test]
async fn writer_document_vcs_replays_text_mutations() {
    let mut store = seeded_store().await;
    store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![WriterMutation::EditText(EditText { text: "hello".into() })], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    assert_eq!(crate::writer_text(&store.snapshot().expect("snapshot")), "hello");
}

#[semio_framework_async_macros::async_test]
async fn writer_document_vcs_undoes_text_mutation() {
    let mut store = seeded_store().await;
    store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![WriterMutation::EditText(EditText { text: "hello".into() })], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    store.dispatch(store::os_store::store_schema::ArtifactCommand::Undo, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("undo");
    assert_eq!(crate::writer_text(&store.snapshot().expect("snapshot")), "");
}

//#region 🔖️MutationLaws
#[semio_framework_async_macros::async_test]
async fn rename_writer_and_edit_text_invert_to_the_prior_field_value() {
    let snapshot = crate::writer_snapshot_with_text(crate::WRITER_DOCUMENT_SCHEMA, "old-id", "plaintext", "writer://old-id", "old text");
    assert_eq!(WriterMutation::RenameWriter(RenameWriter { new_id: "new-id".into() }).inverse(&snapshot).expect("valid retained mutation inverse fixture"), vec![WriterMutation::RenameWriter(RenameWriter { new_id: "old-id".into() })]);
    assert_eq!(WriterMutation::EditText(EditText { text: "new text".into() }).inverse(&snapshot).expect("valid retained mutation inverse fixture"), vec![WriterMutation::EditText(EditText { text: "old text".into() })]);
}

#[semio_framework_async_macros::async_test]
async fn change_uri_and_change_language_obey_the_inverse_and_diff_absorb_laws() {
    let base = WriterSnapshot { uri: "writer://a".into(), language_id: "plaintext".into(), ..schema::empty_writer_snapshot() };

    let uri_mutation = WriterMutation::ChangeUri(ChangeUri { new_uri: "writer://b".into() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &uri_mutation).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&uri_mutation, &base).await;
    let d1 = uri_mutation.diff(&base).diff().clone();
    let d2 = WriterMutation::ChangeUri(ChangeUri { new_uri: "writer://c".into() }).diff(&base).diff().clone();
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;

    let language_mutation = WriterMutation::ChangeLanguage(ChangeLanguage { new_language_id: "jack".into() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &language_mutation).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&language_mutation, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn edit_text_obeys_the_inverse_and_diff_absorb_laws() {
    let base = crate::writer_snapshot_with_text(crate::WRITER_DOCUMENT_SCHEMA, "empty", "plaintext", "writer://empty", "first");
    let mutation = WriterMutation::EditText(EditText { text: "second".into() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = WriterMutation::EditText(EditText { text: "third".into() }).diff(&base).diff().clone();
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
//#endregion 🔖️MutationLaws

//#region 🧪️OutcomeLaws
/// ⚖️ `📋️contract-freeze.md` §C2 laws. Writer's four kinds are whole-document scoped (no
/// addressed sub-element), so `assert_missing_target_is_error` doesn't apply here — every kind's
/// only checkable law is `mutation.no-op` (exercised in `🔖️MutationLaws` above) plus determinism
/// and the per-verb-family `assert_outcome_policy_matrix` below (rename, change/set, edit).
#[semio_framework_async_macros::async_test]
async fn edit_text_outcome_is_deterministic() {
    let base = crate::writer_snapshot_with_text(crate::WRITER_DOCUMENT_SCHEMA, "empty", "plaintext", "writer://empty", "first");
    let mutation = WriterMutation::EditText(EditText { text: "second".into() });
    protocol::os_spr::protocol_laws::assert_outcome_deterministic(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_writer_outcome_obeys_the_policy_matrix() {
    let snapshot = crate::writer_snapshot_with_text(crate::WRITER_DOCUMENT_SCHEMA, "old-id", "plaintext", "writer://old-id", "old text");
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&snapshot, &WriterMutation::RenameWriter(RenameWriter { new_id: "new-id".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn change_uri_and_change_language_outcomes_obey_the_policy_matrix() {
    let base = WriterSnapshot { uri: "writer://a".into(), language_id: "plaintext".into(), ..schema::empty_writer_snapshot() };
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &WriterMutation::ChangeUri(ChangeUri { new_uri: "writer://b".into() })).await;
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &WriterMutation::ChangeLanguage(ChangeLanguage { new_language_id: "jack".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn edit_text_outcome_obeys_the_policy_matrix() {
    let base = crate::writer_snapshot_with_text(crate::WRITER_DOCUMENT_SCHEMA, "empty", "plaintext", "writer://empty", "first");
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &WriterMutation::EditText(EditText { text: "second".into() })).await;
}
//#endregion 🧪️OutcomeLaws
