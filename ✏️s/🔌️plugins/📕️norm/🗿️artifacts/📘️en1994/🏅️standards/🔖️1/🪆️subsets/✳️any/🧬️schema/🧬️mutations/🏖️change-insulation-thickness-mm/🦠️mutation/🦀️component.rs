//! 🔥 `change-insulation-thickness-mm` — sets the En 1994 fire protection insulation thickness [mm] scalar.

use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeInsulationThicknessMm {
    pub new_insulation_thickness_mm: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeInsulationThicknessMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "insulation-thickness-mm", kind: "change-insulation-thickness-mm", record: "ChangedInsulationThicknessMm" };

    async fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change insulation thickness to {}", self.new_insulation_thickness_mm)
    }
}
//#endregion 🔖️Payload
