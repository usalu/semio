//! 🌉 `change-bridge-lane` — sets the En1991 bridge lane count scalar.

use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeBridgeLane {
    pub new_bridge_lane: u8,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeBridgeLane {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "bridge-lane", kind: "change-bridge-lane", record: "ChangedBridgeLane" };

    async fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change bridge lane count to {:?}", self.new_bridge_lane)
    }
}
//#endregion 🔖️Payload
