//! 🐊 `change-tank-height-m` payload — changes the En1998 document's `tank_height_m` (tank height [m]).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeTankHeightM
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTankHeightM {
    pub new_tank_height_m: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeTankHeightM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "tank-height-m", kind: "change-tank-height-m", record: "ChangedTankHeightM" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        crate::artifacts::en1998::mutations::change_tank_height_m::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_tank_height_m::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change tank height [m] to {}", self.new_tank_height_m)
    }
}
//#endregion 🔖️ChangeTankHeightM
