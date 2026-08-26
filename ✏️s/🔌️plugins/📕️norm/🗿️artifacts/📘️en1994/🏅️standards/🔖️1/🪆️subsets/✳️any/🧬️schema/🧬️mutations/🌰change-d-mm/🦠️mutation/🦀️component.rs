//! ⭕ `change-d-mm` — sets the En 1994 shear stud shank diameter d [mm] scalar.

use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeDMm {
    pub new_d_mm: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeDMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "d-mm", kind: "change-d-mm", record: "ChangedDMm" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change stud diameter d to {}", self.new_d_mm)
    }
}
//#endregion 🔖️Payload
