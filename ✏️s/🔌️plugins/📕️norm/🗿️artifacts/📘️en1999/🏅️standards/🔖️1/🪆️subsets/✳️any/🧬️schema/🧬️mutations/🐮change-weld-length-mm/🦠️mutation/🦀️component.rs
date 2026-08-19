//! 🐮 `change-weld-length-mm` payload — changes the En1999 document's `weld_length_mm` (weld length [mm]).

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeWeldLengthMm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeWeldLengthMm {
    pub new_weld_length_mm: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeWeldLengthMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "weld-length-mm", kind: "change-weld-length-mm", record: "ChangedWeldLengthMm" };

    async fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        crate::artifacts::en1999::mutations::change_weld_length_mm::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        crate::artifacts::en1999::mutations::change_weld_length_mm::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change weld length [mm] to {}", self.new_weld_length_mm)
    }
}
//#endregion 🔖️ChangeWeldLengthMm
