//! 🌡️ `change-delta-tk` — sets the En1991 thermal delta scalar.

use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeDeltaTK {
    pub new_delta_t_k: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeDeltaTK {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "delta-tk", kind: "change-delta-tk", record: "ChangedDeltaTk" };

    async fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change thermal delta to {:?}", self.new_delta_t_k)
    }
}
//#endregion 🔖️Payload
