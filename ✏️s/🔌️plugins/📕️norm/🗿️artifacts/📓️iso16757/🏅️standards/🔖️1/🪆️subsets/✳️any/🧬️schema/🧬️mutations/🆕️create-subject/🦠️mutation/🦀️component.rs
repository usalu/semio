//! 🆕️ `create-subject` — brings a new id-keyed dictionary subject into existence.

use crate::artifacts::iso16757::{part_4::Subject, Iso16757Mutation, Iso16757Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSubject {
    pub subject: Subject,
    pub index: Option<usize>,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for CreateSubject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "subject", kind: "create-subject", record: "CreatedSubject" };

    fn diff(&self, base: &Iso16757Snapshot) -> <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create dictionary subject \"{}\"", self.subject.names.preferred.text)
    }
    fn target(&self) -> Vec<String> {
        vec![self.subject.id.clone()]
    }
}
//#endregion 🔖️Payload
