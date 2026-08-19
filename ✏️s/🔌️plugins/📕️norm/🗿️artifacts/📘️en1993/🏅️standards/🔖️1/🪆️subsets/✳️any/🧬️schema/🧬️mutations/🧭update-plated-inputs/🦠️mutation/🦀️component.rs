//! 🧱 `update-plated-inputs` — atomically updates the plated-inputs facet (plated_lambda_p, plated_sigma_ed_mpa are validated together for one EN 1993 check, never one-field-at-a-time).

use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePlatedInputs {
    pub new_plated_lambda_p: f64,
    pub new_plated_sigma_ed_mpa: f64,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdatePlatedInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "plated-inputs", kind: "update-plated-inputs", record: "UpdatedPlatedInputs" };

    async fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Update EN 1993-1-5 plated element buckling inputs".to_string()
    }
}
//#endregion 🔖️Payload
