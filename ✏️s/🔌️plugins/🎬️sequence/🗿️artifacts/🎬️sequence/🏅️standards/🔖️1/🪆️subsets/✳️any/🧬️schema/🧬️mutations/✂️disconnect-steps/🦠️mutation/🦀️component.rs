//! ✂️ Sequence mutation — `DisconnectSteps`: removes a flow edge relationship.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::SequenceSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✂️ `disconnect-steps` payload — edge id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "disconnect-steps")]
pub struct DisconnectSteps {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn disconnect_steps(id: String) -> SequenceMutation {
    SequenceMutation::DisconnectSteps(DisconnectSteps { id })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for DisconnectSteps {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "steps", kind: "disconnect-steps", record: "DisconnectedSteps" };

    fn diff(&self, base: &SequenceSnapshot) -> SequenceDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Disconnect edge \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
