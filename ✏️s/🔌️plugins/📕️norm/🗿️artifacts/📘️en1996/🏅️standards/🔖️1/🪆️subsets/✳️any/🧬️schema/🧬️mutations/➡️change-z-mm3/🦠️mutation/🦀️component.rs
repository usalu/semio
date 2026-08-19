//! ➡️ `change-z-mm3` payload — changes the En1996 document's `z_mm3` (section modulus z [mm3]).

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeZMm3
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeZMm3 {
    pub new_z_mm3: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeZMm3 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "z-mm3", kind: "change-z-mm3", record: "ChangedZMm3" };

    async fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        crate::artifacts::en1996::mutations::change_z_mm3::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        crate::artifacts::en1996::mutations::change_z_mm3::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change section modulus z [mm3] to {}", self.new_z_mm3)
    }
}
//#endregion 🔖️ChangeZMm3
