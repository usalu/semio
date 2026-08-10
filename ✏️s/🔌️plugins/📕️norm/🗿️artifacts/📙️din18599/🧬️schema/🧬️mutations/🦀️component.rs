//! 🧬️ Din18599 artifact — document mutation dispatch.

use crate::artifacts::din18599::diff::{diff_set_snapshot, Din18599Diff};
use crate::artifacts::din18599::Din18599Snapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Din18599Mutation {
    SetSnapshot {
        #[dsl(block)]
        snapshot: Din18599Snapshot,
    },
}

impl Mutation<Din18599Snapshot> for Din18599Mutation {
    type Diff = Din18599Diff;

    fn diff(&self, _snapshot: &Din18599Snapshot) -> Din18599Diff {
        match self {
            Din18599Mutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &Din18599Snapshot) -> Vec<Self> {
        match self {
            Din18599Mutation::SetSnapshot { .. } => vec![Din18599Mutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}

crate::impl_norm_set_snapshot_ops!(Din18599Mutation, Din18599Snapshot);
