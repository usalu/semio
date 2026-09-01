//! 🛢️ `update-silo-shell-inputs` — atomically updates the silo-shell-inputs facet (silo_t_mm, silo_r_mm, shell_sigma_x_ed_mpa, silo_k, silo_gamma_kn_m3, silo_depth_m are validated together for one EN 1993 check, never one-field-at-a-time).



use crate::artifacts::en1993::{En1993Diff, En1993Mutation, En1993Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateSiloShellInputs {
    pub new_silo_t_mm: f64,
    pub new_silo_r_mm: f64,
    pub new_shell_sigma_x_ed_mpa: f64,
    pub new_silo_k: f64,
    pub new_silo_gamma_kn_m3: f64,
    pub new_silo_depth_m: f64,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateSiloShellInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "silo-shell-inputs", kind: "update-silo-shell-inputs", record: "UpdatedSiloShellInputs" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update EN 1993-1-6/-4-1 shell and silo wall inputs".to_string()
    }
}
//#endregion 🔖️Payload
