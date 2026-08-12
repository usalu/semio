//! 🔩 `update-bolt-inputs` — atomically updates the bolt-inputs facet (bolt_f_ed_kn, bolt_n_bolts, bolt_a_s_mm2, bolt_e1_mm, bolt_e2_mm, bolt_d0_mm, bolt_d_mm, bolt_t_mm, bolt_f_u_mpa, bolt_f_ub_mpa are validated together for one EN 1993 check, never one-field-at-a-time).

use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateBoltInputs {
    pub new_bolt_f_ed_kn: f64,
    pub new_bolt_n_bolts: u32,
    pub new_bolt_a_s_mm2: f64,
    pub new_bolt_e1_mm: f64,
    pub new_bolt_e2_mm: f64,
    pub new_bolt_d0_mm: f64,
    pub new_bolt_d_mm: f64,
    pub new_bolt_t_mm: f64,
    pub new_bolt_f_u_mpa: f64,
    pub new_bolt_f_ub_mpa: f64,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateBoltInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "bolt-inputs", kind: "update-bolt-inputs", record: "UpdatedBoltInputs" };

    fn diff(&self, base: &En1993Snapshot) -> <En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update EN 1993-1-8 bolted connection inputs".to_string()
    }
}
//#endregion 🔖️Payload
