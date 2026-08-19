//! 📉 `change-delta-tau-stud-mpa` — sets the En 1994 stud fatigue shear stress range Δτ [MPa] scalar.

use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeDeltaTauStudMpa {
    pub new_delta_tau_stud_mpa: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeDeltaTauStudMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "delta-tau-stud-mpa", kind: "change-delta-tau-stud-mpa", record: "ChangedDeltaTauStudMpa" };

    async fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change stud fatigue stress range Δτ to {}", self.new_delta_tau_stud_mpa)
    }
}
//#endregion 🔖️Payload
