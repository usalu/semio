//! 💪 `update-hss-inputs` — atomically updates the hss-inputs facet (hss_w_el_mm3, hss_f_y_mpa, hss_section_class, hss_m_ed_knm are validated together for one EN 1993 check, never one-field-at-a-time).



use crate::artifacts::en1993::{En1993Diff, En1993Mutation, En1993Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateHssInputs {
    pub new_hss_w_el_mm3: f64,
    pub new_hss_f_y_mpa: f64,
    pub new_hss_section_class: u8,
    pub new_hss_m_ed_knm: f64,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateHssInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "hss-inputs", kind: "update-hss-inputs", record: "UpdatedHssInputs" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update EN 1993-1-12 high-strength steel inputs".to_string()
    }
}
//#endregion 🔖️Payload
