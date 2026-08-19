//! 🟩 `change-fk-mpa` payload — changes the En1996 document's `f_k_mpa` (characteristic compressive strength f_k [MPa]).

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFKMpa
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFKMpa {
    pub new_f_k_mpa: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeFKMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fk-mpa", kind: "change-fk-mpa", record: "ChangedFKMpa" };

    async fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        crate::artifacts::en1996::mutations::change_f_k_mpa::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        crate::artifacts::en1996::mutations::change_f_k_mpa::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change characteristic compressive strength f_k [MPa] to {}", self.new_f_k_mpa)
    }
}
//#endregion 🔖️ChangeFKMpa
