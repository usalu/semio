//! 🏗️ `update-member-properties` — atomically updates the member-properties facet (n_ed_kn, m_ed_knm, v_ed_kn, a_mm2, a_v_mm2, w_pl_mm3, f_y_mpa, f_u_mpa, chi, a_net_mm2, tension_n_ed_kn are validated together for one EN 1993 check, never one-field-at-a-time).

use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateMemberProperties {
    pub new_n_ed_kn: f64,
    pub new_m_ed_knm: f64,
    pub new_v_ed_kn: f64,
    pub new_a_mm2: f64,
    pub new_a_v_mm2: f64,
    pub new_w_pl_mm3: f64,
    pub new_f_y_mpa: f64,
    pub new_f_u_mpa: f64,
    pub new_chi: f64,
    pub new_a_net_mm2: f64,
    pub new_tension_n_ed_kn: f64,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateMemberProperties {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "member-properties", kind: "update-member-properties", record: "UpdatedMemberProperties" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update member properties (forces, section, material)".to_string()
    }
}
//#endregion 🔖️Payload
