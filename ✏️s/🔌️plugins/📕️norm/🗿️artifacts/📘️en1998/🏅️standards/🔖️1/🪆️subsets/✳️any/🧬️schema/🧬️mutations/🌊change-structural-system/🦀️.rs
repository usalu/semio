//! 🌊 `change-structural-system` payload — changes the En1998 document's `structural_system` (structural system).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_structural_system::ChangeStructuralSystem;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeStructuralSystem
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeStructuralSystem {
    pub new_structural_system: String,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeStructuralSystem {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "structural-system", kind: "change-structural-system", record: "ChangedStructuralSystem" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change structural system to \"{}\"", self.new_structural_system)
    }
}
//#endregion 🔖️ChangeStructuralSystem
