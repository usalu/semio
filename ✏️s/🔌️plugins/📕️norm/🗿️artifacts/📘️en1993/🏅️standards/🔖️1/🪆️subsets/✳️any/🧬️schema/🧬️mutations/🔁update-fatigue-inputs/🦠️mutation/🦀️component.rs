//! 🔁 `update-fatigue-inputs` — atomically updates the fatigue-inputs facet (delta_sigma_mpa, fatigue_category, fatigue_method are validated together for one EN 1993 check, never one-field-at-a-time).

use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateFatigueInputs {
    pub new_delta_sigma_mpa: f64,
    pub new_fatigue_category: u8,
    pub new_fatigue_method: String,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateFatigueInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "fatigue-inputs", kind: "update-fatigue-inputs", record: "UpdatedFatigueInputs" };

    fn diff(&self, base: &En1993Snapshot) -> <En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update EN 1993-1-9 fatigue inputs".to_string()
    }
}
//#endregion 🔖️Payload
