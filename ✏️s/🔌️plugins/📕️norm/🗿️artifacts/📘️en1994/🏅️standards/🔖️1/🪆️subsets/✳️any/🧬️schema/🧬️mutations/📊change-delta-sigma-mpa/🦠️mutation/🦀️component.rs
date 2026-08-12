//! 📊 `change-delta-sigma-mpa` — sets the En 1994 bridge fatigue stress range Δσ [MPa] scalar.

use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeDeltaSigmaMpa {
    pub new_delta_sigma_mpa: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeDeltaSigmaMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "delta-sigma-mpa", kind: "change-delta-sigma-mpa", record: "ChangedDeltaSigmaMpa" };

    fn diff(&self, base: &En1994Snapshot) -> <En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change fatigue stress range Δσ to {}", self.new_delta_sigma_mpa)
    }
}
//#endregion 🔖️Payload
