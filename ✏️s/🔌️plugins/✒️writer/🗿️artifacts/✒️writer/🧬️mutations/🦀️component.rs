//! 🧬️ Writer artifact — document mutation dispatch enum.

use crate::artifacts::writer::{WriterDiff, WriterProjection};
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
    SetDocument {
        #[dsl(block)]
        document: WriterProjection,
    },
}

pub fn apply_writer_mutation(projection: &mut WriterProjection, mutation: &WriterMutation) {
    match mutation {
        WriterMutation::SetText { text } => super::set_text::mutation::apply(projection, text),
        WriterMutation::SetDocument { document } => super::set_document::mutation::apply(projection, document),
    }
}

pub fn inverse_writer_mutation(projection: &WriterProjection, mutation: &WriterMutation) -> Vec<WriterMutation> {
    match mutation {
        WriterMutation::SetText { text } => super::set_text::inverse::inverse(projection, text),
        WriterMutation::SetDocument { document } => super::set_document::inverse::inverse(projection, document),
    }
}

impl Mutation<WriterProjection> for WriterMutation {
    type Diff = WriterDiff;

    fn diff(&self, _projection: &WriterProjection) -> Self::Diff {
        match self {
            WriterMutation::SetText { text } => WriterDiff { text: Some(text.clone()), ..Default::default() },
            WriterMutation::SetDocument { document } => WriterDiff { document: Some(document.clone()), ..Default::default() },
        }
    }

    fn inverse(&self, projection: &WriterProjection) -> Vec<Self> {
        inverse_writer_mutation(projection, self)
    }
}

pub use super::set_text::mutation::{set_text, SetText};
pub use super::set_document::mutation::{set_document, SetDocument};
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::writer::engine;

    type WriterStore = store::DocumentStore<WriterProjection, WriterMutation>;

    fn seeded_store() -> WriterStore {
        WriterStore::new(store::create_document_envelope("writer.document", "writer", engine::empty_writer_projection(), None))
    }

    #[test]
    fn writer_document_vcs_replays_text_mutations() {
        let mut store = seeded_store();
        store.dispatch(store::DocumentCommand::Apply { mutations: vec![WriterMutation::SetText { text: "hello".into() }], description: None }).expect("apply");
        assert_eq!(store.projection().expect("projection").text, "hello");
    }

    #[test]
    fn writer_document_vcs_undoes_text_mutation() {
        let mut store = seeded_store();
        store.dispatch(store::DocumentCommand::Apply { mutations: vec![WriterMutation::SetText { text: "hello".into() }], description: None }).expect("apply");
        store.dispatch(store::DocumentCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").text, "");
    }
}
//#endregion 🧪️Tests
