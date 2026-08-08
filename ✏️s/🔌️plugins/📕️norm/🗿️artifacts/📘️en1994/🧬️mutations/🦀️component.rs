//! 🧬️ En1994 artifact — snapshot mutation dispatch.

use crate::artifacts::en1994::diff::{diff_set_snapshot, En1994Diff};
use crate::artifacts::en1994::En1994Snapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum En1994Mutation {
    SetSnapshot { snapshot: En1994Snapshot },
}

impl Mutation<En1994Snapshot> for En1994Mutation {
    type Diff = En1994Diff;

    fn diff(&self, _snapshot: &En1994Snapshot) -> En1994Diff {
        match self {
            Self::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &En1994Snapshot) -> Vec<Self> {
        vec![Self::SetSnapshot { snapshot: snapshot.clone() }]
    }
}
//#endregion 🔖️Mutation
