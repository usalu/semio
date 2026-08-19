//! 🐍 `change-tank-mass-t` payload — changes the En1998 document's `tank_mass_t` (tank mass [t]).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeTankMassT
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTankMassT {
    pub new_tank_mass_t: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeTankMassT {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "tank-mass-t", kind: "change-tank-mass-t", record: "ChangedTankMassT" };

    async fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        crate::artifacts::en1998::mutations::change_tank_mass_t::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_tank_mass_t::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change tank mass [t] to {}", self.new_tank_mass_t)
    }
}
//#endregion 🔖️ChangeTankMassT
