//! 🧬️ En1997 artifact — document mutation dispatch.

use crate::artifacts::en1997::diff::{diff_set_snapshot, En1997Diff};
use crate::artifacts::en1997::En1997Snapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum En1997Mutation {
    SetSnapshot {
        #[dsl(block)]
        snapshot: En1997Snapshot,
    },
}

impl Mutation<En1997Snapshot> for En1997Mutation {
    type Diff = En1997Diff;

    fn diff(&self, _snapshot: &En1997Snapshot) -> En1997Diff {
        match self {
            En1997Mutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &En1997Snapshot) -> Vec<Self> {
        match self {
            En1997Mutation::SetSnapshot { .. } => vec![En1997Mutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}
