//! 🦄 `change-k-foundation` payload — changes the En1998 document's `k_foundation` (foundation stiffness k [kN/m]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_k_foundation::ChangeKFoundation;

//#region 🔖️ChangeKFoundation
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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
