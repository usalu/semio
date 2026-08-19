//! 🗑️ Sequence mutation — `DeleteStep`: removes an id-keyed step (captures cascade — any edge
//! touching this step is severed too, re-`connect`ed by the inverse).
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::SequenceSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗑️ `delete-step` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-step")]
pub struct DeleteStep {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn delete_step(id: String) -> SequenceMutation {
    SequenceMutation::DeleteStep(DeleteStep { id })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for DeleteStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "step", kind: "delete-step", record: "DeletedStep" };

    async fn diff(&self, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete step \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
