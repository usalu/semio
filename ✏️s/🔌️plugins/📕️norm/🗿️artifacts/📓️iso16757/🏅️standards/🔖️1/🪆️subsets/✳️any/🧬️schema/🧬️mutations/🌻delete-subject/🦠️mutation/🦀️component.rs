//! 🗑️ `delete-subject` — removes an id-keyed dictionary subject.

use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteSubject {
    pub id: String,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for DeleteSubject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "subject", kind: "delete-subject", record: "DeletedSubject" };

    async fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete dictionary subject \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
