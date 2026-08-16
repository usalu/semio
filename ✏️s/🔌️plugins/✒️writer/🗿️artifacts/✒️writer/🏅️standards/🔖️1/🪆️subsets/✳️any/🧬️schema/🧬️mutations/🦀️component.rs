//! 🧬️ Writer artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! triad leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<WriterSnapshot>`
//! and `impl protocol::SemanticMutation<WriterSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here. `WriterSnapshot` has exactly five persistent scalar fields
//! (`schema`, `id`, `language_id`, `uri`, `text`) and no id-keyed collections, ordered lists,
//! relationships or hierarchy — the whole vocabulary is document-level scalars per
//! `📓️derivation-rules.md` recipe §1. `schema` is the fixed artifact-schema-id constant
//! (`WRITER_DOCUMENT_SCHEMA`), never user-authored, so it gets no mutation.

use crate::artifacts::writer::WriterDiff;
use crate::artifacts::writer::WriterSnapshot;
use protocol::{Mutation, MutationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// @emoji 🧬️ Typed, invertible, semantic writer document mutation vocabulary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = WriterSnapshot, diff = WriterDiff, schema = "writer.writer")]
pub enum WriterMutation {
    RenameWriter(RenameWriter),
    ChangeUri(ChangeUri),
    ChangeLanguage(ChangeLanguage),
    EditText(EditText),
}

/// 🧮️ Diff-first apply — matches every other migrated facet (`operation.diff(base).apply(base)`,
/// per wave 0's confirmation that `vcs::apply_mutation` is already diff-first under the hood).
pub fn apply_writer_mutation(snapshot: &mut WriterSnapshot, mutation: &WriterMutation) {
    *snapshot = mutation.diff(snapshot).diff().apply(snapshot);
}

pub fn inverse_writer_mutation(snapshot: &WriterSnapshot, mutation: &WriterMutation) -> Vec<WriterMutation> {
    mutation.inverse(snapshot)
}

pub use super::rename_writer::mutation::{rename_writer, RenameWriter};
pub use super::change_uri::mutation::{change_uri, ChangeUri};
pub use super::change_language::mutation::{change_language, ChangeLanguage};
pub use super::edit_text::mutation::{edit_text, EditText};
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::writer::schema;

    type WriterStore = store::ArtifactStore<WriterSnapshot, WriterMutation>;

    fn seeded_store() -> WriterStore {
        WriterStore::new(store::create_document_envelope("writer.document", "writer", schema::empty_writer_snapshot(), None))
    }

    #[test]
    fn writer_document_vcs_replays_text_mutations() {
        let mut store = seeded_store();
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![WriterMutation::EditText(EditText { text: "hello".into() })], description: None }).expect("apply");
        assert_eq!(crate::artifacts::writer::writer_text(&store.snapshot().expect("snapshot")), "hello");
    }

    #[test]
    fn writer_document_vcs_undoes_text_mutation() {
        let mut store = seeded_store();
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![WriterMutation::EditText(EditText { text: "hello".into() })], description: None }).expect("apply");
        store.dispatch(store::ArtifactCommand::Undo).expect("undo");
        assert_eq!(crate::artifacts::writer::writer_text(&store.snapshot().expect("snapshot")), "");
    }

    //#region 🔖️MutationLaws
    #[test]
    fn rename_writer_and_edit_text_invert_to_the_prior_field_value() {
        let snapshot = WriterSnapshot { id: "old-id".into(), document: crate::artifacts::writer::document_child_handle_and_cache("old-id", "old text", "plaintext"), ..schema::empty_writer_snapshot() };
        assert_eq!(
            WriterMutation::RenameWriter(RenameWriter { new_id: "new-id".into() }).inverse(&snapshot),
            vec![WriterMutation::RenameWriter(RenameWriter { new_id: "old-id".into() })]
        );
        assert_eq!(
            WriterMutation::EditText(EditText { text: "new text".into() }).inverse(&snapshot),
            vec![WriterMutation::EditText(EditText { text: "old text".into() })]
        );
    }

    #[test]
    fn change_uri_and_change_language_obey_the_inverse_and_diff_absorb_laws() {
        let base = WriterSnapshot { uri: "writer://a".into(), language_id: "plaintext".into(), ..schema::empty_writer_snapshot() };

        let uri_mutation = WriterMutation::ChangeUri(ChangeUri { new_uri: "writer://b".into() });
        protocol::testkit::assert_mutation_inverse_law(&base, &uri_mutation);
        let d1 = uri_mutation.diff(&base).diff().clone();
        let d2 = WriterMutation::ChangeUri(ChangeUri { new_uri: "writer://c".into() }).diff(&base).diff().clone();
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);

        let language_mutation = WriterMutation::ChangeLanguage(ChangeLanguage { new_language_id: "jack".into() });
        protocol::testkit::assert_mutation_inverse_law(&base, &language_mutation);
    }

    #[test]
    fn edit_text_obeys_the_inverse_and_diff_absorb_laws() {
        let base = WriterSnapshot { document: crate::artifacts::writer::document_child_handle_and_cache("empty", "first", "plaintext"), ..schema::empty_writer_snapshot() };
        let mutation = WriterMutation::EditText(EditText { text: "second".into() });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = WriterMutation::EditText(EditText { text: "third".into() }).diff(&base).diff().clone();
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🔖️MutationLaws

    //#region 🧪️OutcomeLaws
    /// ⚖️ `📋️contract-freeze.md` §C2 laws. Writer's four kinds are whole-document scoped (no
    /// addressed sub-element), so `assert_missing_target_is_error` doesn't apply here — every kind's
    /// only checkable law is `mutation.no-op` (exercised in `🔖️MutationLaws` above) plus determinism.
    /// `assert_outcome_policy_matrix` is not yet landed in `📡️spr/🧪️testkit` — TODO(1-D testkit laws
    /// pending) once it lands.
    #[test]
    fn edit_text_outcome_is_deterministic() {
        let base = WriterSnapshot { document: crate::artifacts::writer::document_child_handle_and_cache("empty", "first", "plaintext"), ..schema::empty_writer_snapshot() };
        let mutation = WriterMutation::EditText(EditText { text: "second".into() });
        protocol::testkit::assert_outcome_deterministic(&base, &mutation);
    }
    //#endregion 🧪️OutcomeLaws
}
//#endregion 🧪️Tests
