//! 🧬️ Iso16757 artifact — document mutation dispatch.

use crate::artifacts::iso16757::diff::{diff_set_snapshot, Iso16757Diff};
use crate::artifacts::iso16757::Iso16757Snapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Iso16757Mutation {
    SetSnapshot {
        #[dsl(block)]
        snapshot: Iso16757Snapshot,
    },
}

impl Mutation<Iso16757Snapshot> for Iso16757Mutation {
    type Diff = Iso16757Diff;

    fn diff(&self, _snapshot: &Iso16757Snapshot) -> Iso16757Diff {
        match self {
            Iso16757Mutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &Iso16757Snapshot) -> Vec<Self> {
        match self {
            Iso16757Mutation::SetSnapshot { .. } => vec![Iso16757Mutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}

crate::impl_norm_set_snapshot_ops!(Iso16757Mutation, Iso16757Snapshot);
