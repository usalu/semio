//! ✂️ `change-f-vk-mpa` payload — changes the En1996 document's `f_vk_mpa` (characteristic shear strength f_vk [MPa]).

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFVkMpa
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFVkMpa {
    pub new_f_vk_mpa: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeFVkMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "f-vk-mpa", kind: "change-f-vk-mpa", record: "ChangedFVkMpa" };

    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        crate::artifacts::en1996::mutations::change_f_vk_mpa::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        crate::artifacts::en1996::mutations::change_f_vk_mpa::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change characteristic shear strength f_vk [MPa] to {}", self.new_f_vk_mpa)
    }
}
//#endregion 🔖️ChangeFVkMpa
