use super::*;
use crate::schema;

/// 🔐️ A store WITH its owner catalog installed. `ArtifactStore::new` installs none, and
/// `reserve_edit_history_slot` then refuses every `Apply` with
/// `edit history insertion requires its exact mutation retirement factory` — the fixture could be
/// read but never mutated or undone (fleet-brief stale-test bucket 2).
async fn seeded_store() -> crate::spr::OwnedWriterStore {
    crate::spr::new_writer_store(store::create_document_envelope(crate::WRITER_DOCUMENT_SCHEMA, "writer", schema::empty_writer_snapshot(), None)).await.expect("valid artifact store fixture")
}

#[semio_framework_async_macros::async_test]
async fn writer_document_vcs_replays_text_mutations() {
    let mut store = seeded_store().await;
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![WriterMutation::EditText(EditText { text: "hello".into() })], description: None }).await.expect("apply");
    assert_eq!(crate::writer_text(&store.snapshot().expect("snapshot")), "hello");
}

#[semio_framework_async_macros::async_test]
async fn writer_document_vcs_undoes_text_mutation() {
    let mut store = seeded_store().await;
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![WriterMutation::EditText(EditText { text: "hello".into() })], description: None }).await.expect("apply");
    store.dispatch(store::ArtifactCommand::Undo).await.expect("undo");
    assert_eq!(crate::writer_text(&store.snapshot().expect("snapshot")), "");
}

//#region 🔖️MutationLaws
#[semio_framework_async_macros::async_test]
async fn rename_writer_and_edit_text_invert_to_the_prior_field_value() {
    // 📝️ Built through the ONE consistent constructor: since `WriterSnapshot::text` became the
    // PERSISTED body (`writer_text` reads that field, not the child handle's local owner), a base that
    // sets only the handle carries an EMPTY body, and `EditText::inverse` then correctly answers
    // `EditText { text: "" }`. `writer_snapshot_with_text` keeps the field and the content-addressed
    // handle in step, which is the only shape a real document ever has.
    let snapshot = crate::writer_snapshot_with_text(crate::WRITER_DOCUMENT_SCHEMA, "old-id", "plaintext", "writer://old-id", "old text");
    assert_eq!(WriterMutation::RenameWriter(RenameWriter { new_id: "new-id".into() }).inverse(&snapshot), vec![WriterMutation::RenameWriter(RenameWriter { new_id: "old-id".into() })]);
    assert_eq!(WriterMutation::EditText(EditText { text: "new text".into() }).inverse(&snapshot), vec![WriterMutation::EditText(EditText { text: "old text".into() })]);
}

#[semio_framework_async_macros::async_test]
async fn change_uri_and_change_language_obey_the_inverse_and_diff_absorb_laws() {
    let base = WriterSnapshot { uri: "writer://a".into(), language_id: "plaintext".into(), ..schema::empty_writer_snapshot() };

    let uri_mutation = WriterMutation::ChangeUri(ChangeUri { new_uri: "writer://b".into() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &uri_mutation).await;
    let d1 = uri_mutation.diff(&base).diff().clone();
    let d2 = WriterMutation::ChangeUri(ChangeUri { new_uri: "writer://c".into() }).diff(&base).diff().clone();
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;

    let language_mutation = WriterMutation::ChangeLanguage(ChangeLanguage { new_language_id: "jack".into() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &language_mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn edit_text_obeys_the_inverse_and_diff_absorb_laws() {
    // 📝️ See the note in `rename_writer_and_edit_text_invert_to_the_prior_field_value`: a base whose
    // `text` field and `document` handle disagree cannot survive the inverse law, because `apply`
    // re-mints the handle from the RESTORED text and gets a different content address than the one
    // the base was hand-built with.
    let base = crate::writer_snapshot_with_text(crate::WRITER_DOCUMENT_SCHEMA, "empty", "plaintext", "writer://empty", "first");
    let mutation = WriterMutation::EditText(EditText { text: "second".into() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
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
    // 📝️ See `rename_writer_and_edit_text_invert_to_the_prior_field_value` for why the base is built
    // through `writer_snapshot_with_text` rather than by hand.
    let base = crate::writer_snapshot_with_text(crate::WRITER_DOCUMENT_SCHEMA, "empty", "plaintext", "writer://empty", "first");
    let mutation = WriterMutation::EditText(EditText { text: "second".into() });
    protocol::os_spr::protocol_laws::assert_outcome_deterministic(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_writer_outcome_obeys_the_policy_matrix() {
    // 📝️ See `rename_writer_and_edit_text_invert_to_the_prior_field_value` for why the base is built
    // through `writer_snapshot_with_text` rather than by hand.
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
    // 📝️ See `rename_writer_and_edit_text_invert_to_the_prior_field_value` for why the base is built
    // through `writer_snapshot_with_text` rather than by hand.
    let base = crate::writer_snapshot_with_text(crate::WRITER_DOCUMENT_SCHEMA, "empty", "plaintext", "writer://empty", "first");
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &WriterMutation::EditText(EditText { text: "second".into() })).await;
}
//#endregion 🧪️OutcomeLaws
