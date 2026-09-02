//! 🐍 `change-tank-mass-t` payload — changes the En1998 document's `tank_mass_t` (tank mass [t]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_tank_mass_t::ChangeTankMassT;

//#region 🔖️ChangeTankMassT
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeTankMassT {
    pub new_tank_mass_t: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeTankMassT {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "tank-mass-t", kind: "change-tank-mass-t", record: "ChangedTankMassT" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change tank mass [t] to {}", self.new_tank_mass_t)
    }
}
//#endregion 🔖️ChangeTankMassT
