//! 🧬️ Vdi3805 artifact — document mutation dispatch.

use crate::artifacts::vdi3805::diff::{diff_set_snapshot, Vdi3805Diff};
use crate::artifacts::vdi3805::Vdi3805Snapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Vdi3805Mutation {
    SetSnapshot {
        #[dsl(block)]
        snapshot: Vdi3805Snapshot,
    },
}

impl Mutation<Vdi3805Snapshot> for Vdi3805Mutation {
    type Diff = Vdi3805Diff;

    fn diff(&self, _snapshot: &Vdi3805Snapshot) -> Vdi3805Diff {
        match self {
            Vdi3805Mutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &Vdi3805Snapshot) -> Vec<Self> {
        match self {
            Vdi3805Mutation::SetSnapshot { .. } => vec![Vdi3805Mutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}

crate::impl_norm_set_snapshot_ops!(Vdi3805Mutation, Vdi3805Snapshot);
