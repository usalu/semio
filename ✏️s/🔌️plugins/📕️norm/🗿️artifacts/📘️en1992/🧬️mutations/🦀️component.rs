//! 🧬️ En1992 artifact — snapshot mutation dispatch.

use crate::artifacts::en1992::diff::{diff_set_snapshot, En1992Diff};
use crate::artifacts::en1992::En1992Snapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum En1992Mutation {
    SetSnapshot { snapshot: En1992Snapshot },
}

impl Mutation<En1992Snapshot> for En1992Mutation {
    type Diff = En1992Diff;

    fn diff(&self, _snapshot: &En1992Snapshot) -> En1992Diff {
        match self {
            Self::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &En1992Snapshot) -> Vec<Self> {
        vec![Self::SetSnapshot { snapshot: snapshot.clone() }]
    }
}
//#endregion 🔖️Mutation
