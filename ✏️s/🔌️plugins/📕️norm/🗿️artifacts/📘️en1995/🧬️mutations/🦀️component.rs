//! 🧬️ En1995 artifact — document mutation dispatch.

use crate::artifacts::en1995::diff::{diff_set_snapshot, En1995Diff};
use crate::artifacts::en1995::En1995Snapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum En1995Mutation {
    SetSnapshot {
        #[dsl(block)]
        snapshot: En1995Snapshot,
    },
}

impl Mutation<En1995Snapshot> for En1995Mutation {
    type Diff = En1995Diff;

    fn diff(&self, _snapshot: &En1995Snapshot) -> En1995Diff {
        match self {
            En1995Mutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &En1995Snapshot) -> Vec<Self> {
        match self {
            En1995Mutation::SetSnapshot { .. } => vec![En1995Mutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}
