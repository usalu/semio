//! 📏 `change-h-sc-mm` — sets the En 1994 shear stud height h_sc [mm] scalar.

use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeHScMm {
    pub new_h_sc_mm: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeHScMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "h-sc-mm", kind: "change-h-sc-mm", record: "ChangedHScMm" };

    async fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change stud height h_sc to {}", self.new_h_sc_mm)
    }
}
//#endregion 🔖️Payload
