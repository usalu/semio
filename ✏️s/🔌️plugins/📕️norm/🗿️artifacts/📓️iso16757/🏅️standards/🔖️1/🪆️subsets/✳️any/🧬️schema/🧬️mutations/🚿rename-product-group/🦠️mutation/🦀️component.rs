//! ✏️ `rename-product-group` — renames one product group's identity field, addressed by id.

use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenameProductGroup {
    pub id: String,
    pub new_name: String,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for RenameProductGroup {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "product-group", kind: "rename-product-group", record: "RenamedProductGroup" };

    async fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename product group \"{}\" to \"{}\"", self.id, self.new_name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
