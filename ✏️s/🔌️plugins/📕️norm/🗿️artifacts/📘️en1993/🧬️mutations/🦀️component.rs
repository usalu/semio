//! 🧬️ En1993 artifact — snapshot mutation dispatch.

use crate::artifacts::en1993::diff::{diff_set_snapshot, En1993Diff};
use crate::artifacts::en1993::En1993Snapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum En1993Mutation {
    SetSnapshot { snapshot: En1993Snapshot },
}

impl Mutation<En1993Snapshot> for En1993Mutation {
    type Diff = En1993Diff;

    fn diff(&self, _snapshot: &En1993Snapshot) -> En1993Diff {
        match self {
            Self::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &En1993Snapshot) -> Vec<Self> {
        vec![Self::SetSnapshot { snapshot: snapshot.clone() }]
    }
}
//#endregion 🔖️Mutation
