//! 🧬️ En1999 artifact — document mutation dispatch.

use crate::artifacts::en1999::diff::{diff_set_snapshot, En1999Diff};
use crate::artifacts::en1999::En1999Snapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum En1999Mutation {
    SetSnapshot {
        #[dsl(block)]
        snapshot: En1999Snapshot,
    },
}

impl Mutation<En1999Snapshot> for En1999Mutation {
    type Diff = En1999Diff;

    fn diff(&self, _snapshot: &En1999Snapshot) -> En1999Diff {
        match self {
            En1999Mutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &En1999Snapshot) -> Vec<Self> {
        match self {
            En1999Mutation::SetSnapshot { .. } => vec![En1999Mutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}

crate::impl_norm_set_snapshot_ops!(En1999Mutation, En1999Snapshot);
