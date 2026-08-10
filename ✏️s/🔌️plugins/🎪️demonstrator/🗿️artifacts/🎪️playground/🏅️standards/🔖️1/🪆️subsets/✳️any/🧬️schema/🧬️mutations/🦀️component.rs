//! 🧬️ Playground artifact — document mutation dispatch enum.

use crate::artifacts::playground::diff::{diff_set_snapshot, PlaygroundDiff};
use crate::artifacts::playground::PlaygroundSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[allow(clippy::large_enum_variant, reason = "SetSnapshot.snapshot carries the whole document")]
pub enum PlaygroundMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        #[dsl(block)]
        snapshot: PlaygroundSnapshot,
    },
}

/// 🧬️ Applies a playground mutation onto a snapshot.
pub fn apply_playground_mutation(snapshot: &mut PlaygroundSnapshot, mutation: &PlaygroundMutation) {
    match mutation {
        PlaygroundMutation::NoMutation => {}
        PlaygroundMutation::SetSnapshot { snapshot: replacement } => *snapshot = replacement.clone(),
    }
}

/// ↩️ Builds the inverse mutations for a playground mutation.
pub fn inverse_playground_mutation(snapshot: &PlaygroundSnapshot, mutation: &PlaygroundMutation) -> Vec<PlaygroundMutation> {
    match mutation {
        PlaygroundMutation::NoMutation => vec![PlaygroundMutation::NoMutation],
        PlaygroundMutation::SetSnapshot { .. } => vec![PlaygroundMutation::SetSnapshot { snapshot: snapshot.clone() }],
    }
}

impl Mutation<PlaygroundSnapshot> for PlaygroundMutation {
    type Diff = PlaygroundDiff;

    fn diff(&self, _snapshot: &PlaygroundSnapshot) -> Self::Diff {
        match self {
            PlaygroundMutation::NoMutation => PlaygroundDiff::default(),
            PlaygroundMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &PlaygroundSnapshot) -> Vec<Self> {
        inverse_playground_mutation(snapshot, self)
    }
}
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::playground::engine;

    #[test]
    fn playground_mutation_round_trips_store() {
        let mut store = store::ArtifactStore::<PlaygroundSnapshot, PlaygroundMutation>::new(store::create_document_envelope(
            "playground.document",
            "playground",
            engine::empty_playground_snapshot(),
            None,
        ));
        let next = PlaygroundSnapshot { schema: "playground.playground".into() };
        store
            .dispatch(store::ArtifactCommand::Apply {
                mutations: vec![PlaygroundMutation::SetSnapshot { snapshot: next.clone() }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.snapshot().expect("snapshot").schema, next.schema);
    }
}
//#endregion 🧪️Tests
