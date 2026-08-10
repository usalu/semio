//! 🧬️ Din4108 artifact — document mutation dispatch.

use crate::artifacts::din4108::diff::{diff_set_snapshot, Din4108Diff};
use crate::artifacts::din4108::Din4108Snapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Din4108Mutation {
    SetSnapshot {
        #[dsl(block)]
        snapshot: Din4108Snapshot,
    },
}

impl Mutation<Din4108Snapshot> for Din4108Mutation {
    type Diff = Din4108Diff;

    fn diff(&self, _snapshot: &Din4108Snapshot) -> Din4108Diff {
        match self {
            Din4108Mutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &Din4108Snapshot) -> Vec<Self> {
        match self {
            Din4108Mutation::SetSnapshot { .. } => vec![Din4108Mutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}

crate::impl_norm_set_snapshot_ops!(Din4108Mutation, Din4108Snapshot);
