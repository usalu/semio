//! 🧬️ Din16798 artifact — document mutation dispatch.

use crate::artifacts::din16798::diff::{diff_set_snapshot, Din16798Diff};
use crate::artifacts::din16798::Din16798Snapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Din16798Mutation {
    SetSnapshot {
        #[dsl(block)]
        snapshot: Din16798Snapshot,
    },
}

impl Mutation<Din16798Snapshot> for Din16798Mutation {
    type Diff = Din16798Diff;

    fn diff(&self, _snapshot: &Din16798Snapshot) -> Din16798Diff {
        match self {
            Din16798Mutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &Din16798Snapshot) -> Vec<Self> {
        match self {
            Din16798Mutation::SetSnapshot { .. } => vec![Din16798Mutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}

crate::impl_norm_set_snapshot_ops!(Din16798Mutation, Din16798Snapshot);
