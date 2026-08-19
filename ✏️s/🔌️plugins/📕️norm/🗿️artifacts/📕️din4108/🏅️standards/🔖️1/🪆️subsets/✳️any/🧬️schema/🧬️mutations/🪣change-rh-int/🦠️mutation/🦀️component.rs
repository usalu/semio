//! 💧 `change-rh-int` — sets the DIN 4108 `rh_int` scalar.

use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeRhInt {
    pub new_rh_int: f64,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeRhInt {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "rh-int", kind: "change-rh-int", record: "ChangedRhInt" };

    async fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change rh int to {}", self.new_rh_int)
    }
}
//#endregion 🔖️Payload
