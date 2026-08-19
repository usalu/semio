//! 🐗 `change-wall-r` payload — changes the En1998 document's `wall_r` (wall behaviour factor r).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeWallR
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeWallR {
    pub new_wall_r: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeWallR {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "wall-r", kind: "change-wall-r", record: "ChangedWallR" };

    async fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        crate::artifacts::en1998::mutations::change_wall_r::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_wall_r::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change wall behaviour factor r to {}", self.new_wall_r)
    }
}
//#endregion 🔖️ChangeWallR
