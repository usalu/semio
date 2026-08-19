//! 🪞 `update-stainless-inputs` — atomically updates the stainless-inputs facet (stainless_m_ed_knm, stainless_w_pl_mm3, stainless_f_y_mpa are validated together for one EN 1993 check, never one-field-at-a-time).

use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateStainlessInputs {
    pub new_stainless_m_ed_knm: f64,
    pub new_stainless_w_pl_mm3: f64,
    pub new_stainless_f_y_mpa: f64,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateStainlessInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "stainless-inputs", kind: "update-stainless-inputs", record: "UpdatedStainlessInputs" };

    async fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Update EN 1993-1-4 stainless steel inputs".to_string()
    }
}
//#endregion 🔖️Payload
