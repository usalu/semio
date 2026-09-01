//! 🦄 `change-k-foundation` payload — changes the En1998 document's `k_foundation` (foundation stiffness k [kN/m]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_k_foundation::ChangeKFoundation;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeKFoundation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeKFoundation {
    pub new_k_foundation: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeKFoundation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "k-foundation", kind: "change-k-foundation", record: "ChangedKFoundation" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change foundation stiffness k [kN/m] to {}", self.new_k_foundation)
    }
}
//#endregion 🔖️ChangeKFoundation
