//! ❄️ `change-storeys` payload — changes the En1996 document's `storeys` (number of storeys).


use crate::artifacts::en1996::En1996Snapshot;
use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::mutations::change_storeys::ChangeStoreys;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeStoreys
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeStoreys {
    pub new_storeys: u32,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeStoreys {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "storeys", kind: "change-storeys", record: "ChangedStoreys" };

    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change number of storeys to {}", self.new_storeys)
    }
}
//#endregion 🔖️ChangeStoreys
