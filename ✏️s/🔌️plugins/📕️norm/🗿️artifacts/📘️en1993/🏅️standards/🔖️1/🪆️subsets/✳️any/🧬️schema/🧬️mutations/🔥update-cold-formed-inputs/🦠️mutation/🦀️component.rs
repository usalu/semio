//! 📐 `update-cold-formed-inputs` — atomically updates the cold-formed-inputs facet (cf_b_bar_mm, cf_t_mm, cf_k_sigma, cf_psi, cf_n_ed_kn, cf_gross_resistance_kn are validated together for one EN 1993 check, never one-field-at-a-time).

use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateColdFormedInputs {
    pub new_cf_b_bar_mm: f64,
    pub new_cf_t_mm: f64,
    pub new_cf_k_sigma: f64,
    pub new_cf_psi: f64,
    pub new_cf_n_ed_kn: f64,
    pub new_cf_gross_resistance_kn: f64,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateColdFormedInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "cold-formed-inputs", kind: "update-cold-formed-inputs", record: "UpdatedColdFormedInputs" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update EN 1993-1-3 cold-formed section inputs".to_string()
    }
}
//#endregion 🔖️Payload
