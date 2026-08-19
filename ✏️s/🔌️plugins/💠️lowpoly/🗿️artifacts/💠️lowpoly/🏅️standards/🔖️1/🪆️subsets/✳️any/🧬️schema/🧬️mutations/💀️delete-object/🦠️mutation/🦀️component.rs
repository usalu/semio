//! 💀️ `delete-object` — removes an id-keyed lowpoly object (mesh, transform and every paint layer
//! it owns are captured wholesale, since they live embedded on the object itself).

use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteObject {
    pub id: String,
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for DeleteObject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "object", kind: "delete-object", record: "DeletedObject" };

    async fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete object \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
