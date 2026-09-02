//! ✂️ `change-f-vk-mpa` payload — changes the En1996 document's `f_vk_mpa` (characteristic shear strength f_vk [MPa]).


use crate::artifacts::en1996::En1996Snapshot;
use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::mutations::change_f_vk_mpa::ChangeFVkMpa;

//#region 🔖️ChangeFVkMpa
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeFVkMpa {
    pub new_f_vk_mpa: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeFVkMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "f-vk-mpa", kind: "change-f-vk-mpa", record: "ChangedFVkMpa" };

    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change characteristic shear strength f_vk [MPa] to {}", self.new_f_vk_mpa)
    }
}
//#endregion 🔖️ChangeFVkMpa
