//! 🕷️ `change-tower-is-chimney` payload — changes the En1998 document's `tower_is_chimney` (tower-is-chimney flag).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_tower_is_chimney::ChangeTowerIsChimney;

//#region 🔖️ChangeTowerIsChimney
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeTowerIsChimney {
    pub new_tower_is_chimney: bool,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeTowerIsChimney {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "tower-is-chimney", kind: "change-tower-is-chimney", record: "ChangedTowerIsChimney" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change tower-is-chimney flag to {}", self.new_tower_is_chimney)
    }
}
//#endregion 🔖️ChangeTowerIsChimney
