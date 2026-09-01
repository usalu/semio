//! 🦎 `change-tank-radius-m` payload — changes the En1998 document's `tank_radius_m` (tank radius [m]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_tank_radius_m::ChangeTankRadiusM;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeTankRadiusM
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTankRadiusM {
    pub new_tank_radius_m: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeTankRadiusM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "tank-radius-m", kind: "change-tank-radius-m", record: "ChangedTankRadiusM" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change tank radius [m] to {}", self.new_tank_radius_m)
    }
}
//#endregion 🔖️ChangeTankRadiusM
