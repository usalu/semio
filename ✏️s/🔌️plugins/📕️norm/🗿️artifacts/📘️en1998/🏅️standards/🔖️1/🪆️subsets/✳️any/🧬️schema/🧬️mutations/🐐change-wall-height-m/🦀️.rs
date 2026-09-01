//! 🐐 `change-wall-height-m` payload — changes the En1998 document's `wall_height_m` (retaining wall height [m]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_wall_height_m::ChangeWallHeightM;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeWallHeightM
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeWallHeightM {
    pub new_wall_height_m: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeWallHeightM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "wall-height-m", kind: "change-wall-height-m", record: "ChangedWallHeightM" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change retaining wall height [m] to {}", self.new_wall_height_m)
    }
}
//#endregion 🔖️ChangeWallHeightM
