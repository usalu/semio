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

    fn diff(&self, base: &Iso16757Snapshot) -> <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete dictionary subject \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
