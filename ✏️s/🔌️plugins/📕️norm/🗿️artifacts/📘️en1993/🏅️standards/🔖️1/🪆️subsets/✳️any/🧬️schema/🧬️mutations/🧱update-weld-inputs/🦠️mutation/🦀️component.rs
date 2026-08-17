//! ⚡ `update-weld-inputs` — atomically updates the weld-inputs facet (weld_a_mm, weld_l_mm, weld_f_u_mpa, weld_steel_grade, weld_f_ed_kn are validated together for one EN 1993 check, never one-field-at-a-time).

use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateWeldInputs {
    pub new_weld_a_mm: f64,
    pub new_weld_l_mm: f64,
    pub new_weld_f_u_mpa: f64,
    pub new_weld_steel_grade: String,
    pub new_weld_f_ed_kn: f64,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateWeldInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "weld-inputs", kind: "update-weld-inputs", record: "UpdatedWeldInputs" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update EN 1993-1-8 welded connection inputs".to_string()
    }
}
//#endregion 🔖️Payload
