//! 🔧 `change-h-mm` payload — changes the En1995 document's `h_mm` (EN 1995 input).

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeHMm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeHMm {
    pub new_h_mm: f64,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeHMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "h-mm", kind: "change-h-mm", record: "ChangedHMm" };

    async fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        crate::artifacts::en1995::mutations::change_h_mm::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        crate::artifacts::en1995::mutations::change_h_mm::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change h mm to {:?}", self.new_h_mm)
    }
}
//#endregion 🔖️ChangeHMm
