//! 🏜️ `change-bridge-v-rd-kn` payload — changes the En1998 document's `bridge_v_rd_kn` (bridge design shear resistance [kN]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_bridge_v_rd_kn::ChangeBridgeVRdKn;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeBridgeVRdKn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeBridgeVRdKn {
    pub new_bridge_v_rd_kn: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeBridgeVRdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "bridge-v-rd-kn", kind: "change-bridge-v-rd-kn", record: "ChangedBridgeVRdKn" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change bridge design shear resistance [kN] to {}", self.new_bridge_v_rd_kn)
    }
}
//#endregion 🔖️ChangeBridgeVRdKn
