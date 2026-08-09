//! 🧬️ En1991 artifact — snapshot mutation dispatch.

use crate::artifacts::en1991::diff::{diff_set_snapshot, En1991Diff};
use crate::artifacts::en1991::En1991Snapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum En1991Mutation {
    SetSnapshot {
        #[dsl(block)]
        snapshot: En1991Snapshot,
    },
}

impl Mutation<En1991Snapshot> for En1991Mutation {
    type Diff = En1991Diff;

    fn diff(&self, _snapshot: &En1991Snapshot) -> En1991Diff {
        match self {
            Self::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &En1991Snapshot) -> Vec<Self> {
        vec![Self::SetSnapshot { snapshot: snapshot.clone() }]
    }
}
//#endregion 🔖️Mutation

crate::impl_norm_set_snapshot_ops!(En1991Mutation, En1991Snapshot);
