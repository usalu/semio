//! 🐜 `change-tower-q-nominal` payload — changes the En1998 document's `tower_q_nominal` (tower nominal behaviour factor q).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_tower_q_nominal::ChangeTowerQNominal;

//#region 🔖️ChangeTowerQNominal
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeTowerQNominal {
    pub new_tower_q_nominal: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeTowerQNominal {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "tower-q-nominal", kind: "change-tower-q-nominal", record: "ChangedTowerQNominal" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change tower nominal behaviour factor q to {}", self.new_tower_q_nominal)
    }
}
//#endregion 🔖️ChangeTowerQNominal
