//! 🌬️ `change-mortar` payload — changes the En1996 document's `mortar` (mortar compressive-strength class).


use crate::artifacts::en1996::En1996Snapshot;
use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::mutations::change_mortar::ChangeMortar;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeMortar
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMortar {
    pub new_mortar: crate::artifacts::en1996::part_2::MortarClass,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeMortar {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "mortar", kind: "change-mortar", record: "ChangedMortar" };

    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change mortar compressive-strength class to {:?}", self.new_mortar)
    }
}
//#endregion 🔖️ChangeMortar
