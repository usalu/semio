//! 🟩 `change-fk-mpa` payload — changes the En1996 document's `f_k_mpa` (characteristic compressive strength f_k [MPa]).


use crate::artifacts::en1996::En1996Snapshot;
use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::mutations::change_f_k_mpa::ChangeFKMpa;

//#region 🔖️ChangeFKMpa
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeFKMpa {
    pub new_f_k_mpa: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeFKMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fk-mpa", kind: "change-fk-mpa", record: "ChangedFKMpa" };

    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change characteristic compressive strength f_k [MPa] to {}", self.new_f_k_mpa)
    }
}
//#endregion 🔖️ChangeFKMpa
