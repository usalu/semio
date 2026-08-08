//! 🧬️ En1998 artifact — document mutation dispatch.

use crate::artifacts::en1998::diff::{diff_set_snapshot, En1998Diff};
use crate::artifacts::en1998::En1998Snapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum En1998Mutation {
    SetSnapshot {
        #[dsl(block)]
        snapshot: En1998Snapshot,
    },
}

impl Mutation<En1998Snapshot> for En1998Mutation {
    type Diff = En1998Diff;

    fn diff(&self, _snapshot: &En1998Snapshot) -> En1998Diff {
        match self {
            En1998Mutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &En1998Snapshot) -> Vec<Self> {
        match self {
            En1998Mutation::SetSnapshot { .. } => vec![En1998Mutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}
