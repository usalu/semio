//! 🔗 Sequence mutation — `ConnectSteps`: creates a flow edge relationship between two steps.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::SequenceSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔗 `connect-steps` payload — edge `id` plus both endpoint step ids.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "connect-steps")]
pub struct ConnectSteps {
    pub id: String,
    pub from: String,
    pub to: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn connect_steps(id: String, from: String, to: String) -> SequenceMutation {
    SequenceMutation::ConnectSteps(ConnectSteps { id, from, to })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for ConnectSteps {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "steps", kind: "connect-steps", record: "ConnectedSteps" };

    async fn diff(&self, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Connect step \"{}\" to \"{}\"", self.from, self.to)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
