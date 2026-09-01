//! 🦔 `change-tower-mass-t` payload — changes the En1998 document's `tower_mass_t` (tower mass [t]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_tower_mass_t::ChangeTowerMassT;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeTowerMassT
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTowerMassT {
    pub new_tower_mass_t: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeTowerMassT {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "tower-mass-t", kind: "change-tower-mass-t", record: "ChangedTowerMassT" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change tower mass [t] to {}", self.new_tower_mass_t)
    }
}
//#endregion 🔖️ChangeTowerMassT
