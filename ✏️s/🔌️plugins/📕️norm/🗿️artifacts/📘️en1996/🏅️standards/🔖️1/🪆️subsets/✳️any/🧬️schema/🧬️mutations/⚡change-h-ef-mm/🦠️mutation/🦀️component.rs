//! ⚡ `change-h-ef-mm` payload — changes the En1996 document's `h_ef_mm` (effective height h_ef [mm]).

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeHEfMm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeHEfMm {
    pub new_h_ef_mm: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeHEfMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "h-ef-mm", kind: "change-h-ef-mm", record: "ChangedHEfMm" };

    fn diff(&self, base: &En1996Snapshot) -> En1996Diff {
        crate::artifacts::en1996::mutations::change_h_ef_mm::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        crate::artifacts::en1996::mutations::change_h_ef_mm::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change effective height h_ef [mm] to {}", self.new_h_ef_mm)
    }
}
//#endregion 🔖️ChangeHEfMm
