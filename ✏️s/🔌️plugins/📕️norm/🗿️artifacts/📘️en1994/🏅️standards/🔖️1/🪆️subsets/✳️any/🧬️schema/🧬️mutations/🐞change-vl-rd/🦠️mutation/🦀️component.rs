//! 🪛 `change-vl-rd` — sets the En 1994 longitudinal shear resistance V_L,Rd [kN] scalar.

use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeVLRd {
    pub new_v_l_rd: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeVLRd {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vl-rd", kind: "change-vl-rd", record: "ChangedVLRd" };

    async fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change longitudinal shear resistance V_L,Rd to {}", self.new_v_l_rd)
    }
}
//#endregion 🔖️Payload
