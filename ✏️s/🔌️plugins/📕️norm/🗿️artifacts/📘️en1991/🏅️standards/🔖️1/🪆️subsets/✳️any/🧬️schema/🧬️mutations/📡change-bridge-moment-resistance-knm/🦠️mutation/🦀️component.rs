//! 🔩 `change-bridge-moment-resistance-knm` — sets the En1991 bridge moment resistance scalar.

use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeBridgeMomentResistanceKnm {
    pub new_bridge_moment_resistance_knm: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeBridgeMomentResistanceKnm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "bridge-moment-resistance-knm", kind: "change-bridge-moment-resistance-knm", record: "ChangedBridgeMomentResistanceKnm" };

    async fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change bridge moment resistance to {:?}", self.new_bridge_moment_resistance_knm)
    }
}
//#endregion 🔖️Payload
