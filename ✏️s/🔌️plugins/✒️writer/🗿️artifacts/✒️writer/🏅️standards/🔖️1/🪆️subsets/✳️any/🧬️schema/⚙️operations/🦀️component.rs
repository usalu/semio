//! ⚙️ Writer mutation application, codec bridge, store laws, and behavior tests.

use crate::artifacts::writer::schema::mutations::{ChangeLanguage, ChangeUri, EditText, RenameWriter, WriterMutation};
use crate::artifacts::writer::WriterDiff;
use crate::artifacts::writer::WriterSnapshot;
use protocol::{Mutation, MutationDiff};

//#region ⚙️Operations
/// 🧮️ Diff-first apply — matches every other migrated facet (`operation.diff(base).apply(base)`,
/// per wave 0's confirmation that `vcs::apply_mutation` is already diff-first under the hood).
pub fn apply_writer_mutation(snapshot: &mut WriterSnapshot, mutation: &WriterMutation) -> protocol::MutationApplyResult<()> {
    let next = mutation.diff(snapshot).diff().apply(snapshot)?;

    *snapshot = next;
    Ok(())
}

pub fn inverse_writer_mutation(snapshot: &WriterSnapshot, mutation: &WriterMutation) -> Vec<WriterMutation> {
    mutation.inverse(snapshot)
}

/// 🧮️ Applies `mutation` to `snapshot` and hands back the whole [`protocol::MutationOutcome`], the
/// diagnostics included. [`apply_writer_mutation`] answers `Result<(), _>` and drops the messages,
/// so a caller that has to distinguish an applied edit from an applied-with-`mutation.no-op`-warning
/// one — which is exactly what `edit-text`'s committed vector declares — cannot use it.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn apply_writer_mutation_outcome(snapshot: &mut WriterSnapshot, mutation: &WriterMutation) -> protocol::MutationOutcome<WriterDiff> {
    let outcome = <WriterMutation as protocol::Mutation<WriterSnapshot>>::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ `mutation`'s own inverse against `base`, as the step LIST `protocol::Mutation::inverse`
/// returns. Reachable from outside this crate, which `protocol::Mutation` itself is not — the
/// `protocol` extern-crate alias is private to `📦️glue.rs`.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn inverse_writer_mutation_steps(mutation: &WriterMutation, base: &WriterSnapshot) -> Vec<WriterMutation> {
    mutation.inverse(base)
}

/// 📥️ Decodes the internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) projection the
/// committed `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️component.json` vectors carry.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_writer_mutation_json(text: &str) -> Result<WriterMutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📥️ Decodes a committed `📸️snapshot/{⬅️before,➡️after}/🔣️component.json` vector.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_writer_snapshot_json(text: &str) -> Result<WriterSnapshot, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📤️ The snapshot as the same canonical JSON the committed vectors are written in — the
/// projection an external test host compares through.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn encode_writer_snapshot_json(snapshot: &WriterSnapshot) -> String {
    serde_json::to_string(snapshot).expect("a WriterSnapshot is always serializable")
}

//#endregion ⚙️Operations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::writer::schema;

    type WriterStore = store::ArtifactStore<WriterSnapshot, WriterMutation>;

    fn seeded_store() -> WriterStore {
        WriterStore::new(store::create_document_envelope("writer.document", "writer", schema::empty_writer_snapshot(), None)).expect("valid artifact store fixture")
    }

    #[semio_framework_async_macros::async_test]
    async fn writer_document_vcs_replays_text_mutations() {
        let mut store = seeded_store();
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![WriterMutation::EditText(EditText { text: "hello".into() })], description: None }).expect("apply");
        assert_eq!(crate::artifacts::writer::writer_text(&store.snapshot().expect("snapshot")), "hello");
    }

    #[semio_framework_async_macros::async_test]
    async fn writer_document_vcs_undoes_text_mutation() {
        let mut store = seeded_store();
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![WriterMutation::EditText(EditText { text: "hello".into() })], description: None }).expect("apply");
        store.dispatch(store::ArtifactCommand::Undo).expect("undo");
        assert_eq!(crate::artifacts::writer::writer_text(&store.snapshot().expect("snapshot")), "");
    }

    //#region 🔖️MutationLaws
    #[semio_framework_async_macros::async_test]
    async fn rename_writer_and_edit_text_invert_to_the_prior_field_value() {
        let snapshot = WriterSnapshot { id: "old-id".into(), document: crate::artifacts::writer::document_child_handle_with_text("old-id", "old text", "plaintext"), ..schema::empty_writer_snapshot() };
        assert_eq!(WriterMutation::RenameWriter(RenameWriter { new_id: "new-id".into() }).inverse(&snapshot), vec![WriterMutation::RenameWriter(RenameWriter { new_id: "old-id".into() })]);
        assert_eq!(WriterMutation::EditText(EditText { text: "new text".into() }).inverse(&snapshot), vec![WriterMutation::EditText(EditText { text: "old text".into() })]);
    }

    #[semio_framework_async_macros::async_test]
    async fn change_uri_and_change_language_obey_the_inverse_and_diff_absorb_laws() {
        let base = WriterSnapshot { uri: "writer://a".into(), language_id: "plaintext".into(), ..schema::empty_writer_snapshot() };

        let uri_mutation = WriterMutation::ChangeUri(ChangeUri { new_uri: "writer://b".into() });
        protocol::testkit::assert_mutation_inverse_law(&base, &uri_mutation);
        let d1 = uri_mutation.diff(&base).diff().clone();
        let d2 = WriterMutation::ChangeUri(ChangeUri { new_uri: "writer://c".into() }).diff(&base).diff().clone();
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);

        let language_mutation = WriterMutation::ChangeLanguage(ChangeLanguage { new_language_id: "jack".into() });
        protocol::testkit::assert_mutation_inverse_law(&base, &language_mutation);
    }

    #[semio_framework_async_macros::async_test]
    async fn edit_text_obeys_the_inverse_and_diff_absorb_laws() {
        let base = WriterSnapshot { document: crate::artifacts::writer::document_child_handle_with_text("empty", "first", "plaintext"), ..schema::empty_writer_snapshot() };
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
    #[semio_framework_async_macros::async_test]
    async fn edit_text_outcome_is_deterministic() {
        let base = WriterSnapshot { document: crate::artifacts::writer::document_child_handle_with_text("empty", "first", "plaintext"), ..schema::empty_writer_snapshot() };
        let mutation = WriterMutation::EditText(EditText { text: "second".into() });
        protocol::testkit::assert_outcome_deterministic(&base, &mutation);
    }
    //#endregion 🧪️OutcomeLaws
}
//#endregion 🧪️Tests
