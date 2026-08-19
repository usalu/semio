//! 🔧 `change-b-mm` payload — changes the En1992 document's `b_mm` (EN 1992 input).

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeBMm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeBMm {
    pub new_b_mm: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeBMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "b-mm", kind: "change-b-mm", record: "ChangedBMm" };

    async fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        crate::artifacts::en1992::mutations::change_b_mm::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        crate::artifacts::en1992::mutations::change_b_mm::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change b mm to {:?}", self.new_b_mm)
    }
}
//#endregion 🔖️ChangeBMm
