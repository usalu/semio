//! 🦟 `change-w-el-mm3` payload — changes the En1999 document's `w_el_mm3` (elastic section modulus W_el [mm3]).

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeWElMm3
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeWElMm3 {
    pub new_w_el_mm3: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeWElMm3 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "w-el-mm3", kind: "change-w-el-mm3", record: "ChangedWElMm3" };

    async fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        crate::artifacts::en1999::mutations::change_w_el_mm3::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        crate::artifacts::en1999::mutations::change_w_el_mm3::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change elastic section modulus W_el [mm3] to {}", self.new_w_el_mm3)
    }
}
//#endregion 🔖️ChangeWElMm3
