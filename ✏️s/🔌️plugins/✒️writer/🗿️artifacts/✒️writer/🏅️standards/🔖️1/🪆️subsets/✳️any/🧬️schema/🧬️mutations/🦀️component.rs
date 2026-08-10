//! 🧬️ Writer artifact — document mutation dispatch enum.

use crate::artifacts::writer::schema::diff::text::{diff_set_snapshot, diff_set_text};
use crate::artifacts::writer::WriterDiff;
use crate::artifacts::writer::WriterSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// @emoji 🧬️ Typed writer document mutation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum WriterMutation {
    SetText {
        text: String,
    },
    SetSnapshot {
        #[dsl(block)]
        snapshot: WriterSnapshot,
    },
}

pub fn apply_writer_mutation(snapshot: &mut WriterSnapshot, mutation: &WriterMutation) {
    match mutation {
        WriterMutation::SetText { text } => super::set_text::mutation::apply(snapshot, text),
        WriterMutation::SetSnapshot { snapshot: replacement } => super::set_snapshot::mutation::apply(snapshot, replacement),
    }
}

pub fn inverse_writer_mutation(snapshot: &WriterSnapshot, mutation: &WriterMutation) -> Vec<WriterMutation> {
    match mutation {
        WriterMutation::SetText { text } => super::set_text::inverse::inverse(snapshot, text),
        WriterMutation::SetSnapshot { snapshot: replacement } => super::set_snapshot::inverse::inverse(snapshot, replacement),
    }
}

impl Mutation<WriterSnapshot> for WriterMutation {
    type Diff = WriterDiff;

    fn diff(&self, snapshot: &WriterSnapshot) -> Self::Diff {
        match self {
            WriterMutation::SetText { text } => diff_set_text(text),
            WriterMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &WriterSnapshot) -> Vec<Self> {
        inverse_writer_mutation(snapshot, self)
    }
}

pub use super::set_text::mutation::{set_text, SetText};
pub use super::set_snapshot::mutation::{set_snapshot, SetSnapshot};
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::writer::engine;

    type WriterStore = store::DocumentStore<WriterSnapshot, WriterMutation>;

    fn seeded_store() -> WriterStore {
        WriterStore::new(store::create_document_envelope("writer.document", "writer", engine::empty_writer_snapshot(), None))
    }

    #[test]
    fn writer_document_vcs_replays_text_mutations() {
        let mut store = seeded_store();
        store.dispatch(store::DocumentCommand::Apply { mutations: vec![WriterMutation::SetText { text: "hello".into() }], description: None }).expect("apply");
        assert_eq!(store.snapshot().expect("snapshot").text, "hello");
    }

    #[test]
    fn writer_document_vcs_undoes_text_mutation() {
        let mut store = seeded_store();
        store.dispatch(store::DocumentCommand::Apply { mutations: vec![WriterMutation::SetText { text: "hello".into() }], description: None }).expect("apply");
        store.dispatch(store::DocumentCommand::Undo).expect("undo");
        assert_eq!(store.snapshot().expect("snapshot").text, "");
    }
}
//#endregion 🧪️Tests
