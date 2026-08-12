//! ↔️ `change-bridge-lane-width-m` — sets the En1991 bridge lane width scalar.

use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeBridgeLaneWidthM {
    pub new_bridge_lane_width_m: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeBridgeLaneWidthM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "bridge-lane-width-m", kind: "change-bridge-lane-width-m", record: "ChangedBridgeLaneWidthM" };

    fn diff(&self, base: &En1991Snapshot) -> <En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change bridge lane width to {:?}", self.new_bridge_lane_width_m)
    }
}
//#endregion 🔖️Payload
