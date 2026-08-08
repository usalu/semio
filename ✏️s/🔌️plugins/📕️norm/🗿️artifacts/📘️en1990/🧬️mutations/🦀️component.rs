//! 🧬️ En1990 artifact — snapshot mutation dispatch.

use crate::artifacts::en1990::diff::{diff_set_snapshot, En1990Diff};
use crate::artifacts::en1990::En1990Snapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum En1990Mutation {
    SetSnapshot { snapshot: En1990Snapshot },
}

impl Mutation<En1990Snapshot> for En1990Mutation {
    type Diff = En1990Diff;

    fn diff(&self, _snapshot: &En1990Snapshot) -> En1990Diff {
        match self {
            Self::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &En1990Snapshot) -> Vec<Self> {
        vec![Self::SetSnapshot { snapshot: snapshot.clone() }]
    }
}
//#endregion 🔖️Mutation
