//! 🛣️ `change-bridge-span-m` — sets the En1991 bridge span scalar.

use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeBridgeSpanM {
    pub new_bridge_span_m: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeBridgeSpanM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "bridge-span-m", kind: "change-bridge-span-m", record: "ChangedBridgeSpanM" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change bridge span to {:?}", self.new_bridge_span_m)
    }
}
//#endregion 🔖️Payload
