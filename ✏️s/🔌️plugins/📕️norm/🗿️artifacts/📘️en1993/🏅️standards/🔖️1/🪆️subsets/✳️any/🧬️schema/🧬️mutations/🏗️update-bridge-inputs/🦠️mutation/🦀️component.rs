//! 🌉 `update-bridge-inputs` — atomically updates the bridge-inputs facet (bridge_lambda, bridge_phi_2, bridge_delta_sigma_p_mpa are validated together for one EN 1993 check, never one-field-at-a-time).

use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateBridgeInputs {
    pub new_bridge_lambda: f64,
    pub new_bridge_phi_2: f64,
    pub new_bridge_delta_sigma_p_mpa: f64,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateBridgeInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "bridge-inputs", kind: "update-bridge-inputs", record: "UpdatedBridgeInputs" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update EN 1993-2 steel bridge inputs".to_string()
    }
}
//#endregion 🔖️Payload
