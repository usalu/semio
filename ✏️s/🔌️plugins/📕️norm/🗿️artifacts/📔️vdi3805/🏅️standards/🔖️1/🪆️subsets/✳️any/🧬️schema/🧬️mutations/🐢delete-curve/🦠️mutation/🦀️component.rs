//! 🗑️ `delete-curve` — removes an id-keyed characteristic curve.

use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteCurve {
    pub id: String,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for DeleteCurve {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "curve", kind: "delete-curve", record: "DeletedCurve" };

    async fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete curve \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
