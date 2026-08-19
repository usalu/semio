//! 🔆 `change-t-ef-mm` payload — changes the En1996 document's `t_ef_mm` (effective thickness t_ef [mm]).

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeTEfMm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTEfMm {
    pub new_t_ef_mm: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeTEfMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "t-ef-mm", kind: "change-t-ef-mm", record: "ChangedTEfMm" };

    async fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        crate::artifacts::en1996::mutations::change_t_ef_mm::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        crate::artifacts::en1996::mutations::change_t_ef_mm::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change effective thickness t_ef [mm] to {}", self.new_t_ef_mm)
    }
}
//#endregion 🔖️ChangeTEfMm
