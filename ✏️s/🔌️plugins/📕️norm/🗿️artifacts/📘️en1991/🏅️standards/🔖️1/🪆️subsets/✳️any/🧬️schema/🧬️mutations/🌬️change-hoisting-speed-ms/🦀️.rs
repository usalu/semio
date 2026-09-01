//! ⬆️ `change-hoisting-speed-ms` — sets the En1991 hoisting speed scalar.


use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeHoistingSpeedMS {
    pub new_hoisting_speed_m_s: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeHoistingSpeedMS {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "hoisting-speed-ms", kind: "change-hoisting-speed-ms", record: "ChangedHoistingSpeedMs" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change hoisting speed to {:?}", self.new_hoisting_speed_m_s)
    }
}
//#endregion 🔖️Payload
