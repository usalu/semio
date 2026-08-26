//! ⚖️ `change-assumed-gk-kn-m2` — sets the En1991 assumed self-weight load scalar.

use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeAssumedGKKnM2 {
    pub new_assumed_g_k_kn_m2: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeAssumedGKKnM2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "assumed-gk-kn-m2", kind: "change-assumed-gk-kn-m2", record: "ChangedAssumedGkKnM2" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change assumed self-weight load to {:?}", self.new_assumed_g_k_kn_m2)
    }
}
//#endregion 🔖️Payload
