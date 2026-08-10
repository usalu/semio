//! 🧬️ En1996 artifact — document mutation dispatch.

use crate::artifacts::en1996::diff::{diff_set_snapshot, En1996Diff};
use crate::artifacts::en1996::En1996Snapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum En1996Mutation {
    SetSnapshot {
        #[dsl(block)]
        snapshot: En1996Snapshot,
    },
}

impl Mutation<En1996Snapshot> for En1996Mutation {
    type Diff = En1996Diff;

    fn diff(&self, _snapshot: &En1996Snapshot) -> En1996Diff {
        match self {
            En1996Mutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &En1996Snapshot) -> Vec<Self> {
        match self {
            En1996Mutation::SetSnapshot { .. } => vec![En1996Mutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}

crate::impl_norm_set_snapshot_ops!(En1996Mutation, En1996Snapshot);
