//! 🏭 `update-crane-inputs` — atomically updates the crane-inputs facet (crane_f_z_ed_kn, crane_wheel_contact_length_mm, crane_dispersion_mm, crane_t_w_mm are validated together for one EN 1993 check, never one-field-at-a-time).

use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateCraneInputs {
    pub new_crane_f_z_ed_kn: f64,
    pub new_crane_wheel_contact_length_mm: f64,
    pub new_crane_dispersion_mm: f64,
    pub new_crane_t_w_mm: f64,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateCraneInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "crane-inputs", kind: "update-crane-inputs", record: "UpdatedCraneInputs" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update EN 1993-6 crane runway inputs".to_string()
    }
}
//#endregion 🔖️Payload
