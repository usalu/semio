//! ✂️ Sequence mutation — `DisconnectSteps`: removes a flow edge relationship.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::schema::operations::{SequenceDetectedMutation, SequenceDetectionContext};
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
pub async fn disconnect_steps(id: String) -> SequenceMutation {
    SequenceMutation::DisconnectSteps(DisconnectSteps { id })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for DisconnectSteps {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "steps", kind: "disconnect-steps", record: "DisconnectedSteps" };

    async fn diff(&self, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Disconnect edge \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🔎️Detection
/// 🔎️ Detects this leaf's contribution to a before/after sequence plan.
pub fn detect(context: &SequenceDetectionContext<'_>) -> Vec<SequenceDetectedMutation> {
    let removed = context
        .before
        .edges
        .iter()
        .enumerate()
        .filter(|(_, edge)| !context.after_edges.contains_key(edge.id.as_str()) && ![edge.from.as_str(), edge.to.as_str()].iter().any(|id| context.before_steps.contains_key(id) && !context.after_steps.contains_key(id)))
        .map(|(index, edge)| SequenceDetectedMutation { order: (2, index, 0), mutation: SequenceMutation::DisconnectSteps(DisconnectSteps { id: edge.id.clone() }) });
    let retargeted = context
        .after
        .edges
        .iter()
        .enumerate()
        .filter(|(_, edge)| context.before_edges.get(edge.id.as_str()).is_some_and(|before| before.from != edge.from || before.to != edge.to))
        .map(|(index, edge)| SequenceDetectedMutation { order: (3, index, 0), mutation: SequenceMutation::DisconnectSteps(DisconnectSteps { id: edge.id.clone() }) });
    removed.chain(retargeted).collect()
}
//#endregion 🔎️Detection

//#region 🧪️MutationLaws
#[cfg(test)]
mod mutation_law_tests {
    use super::*;
    use crate::artifacts::sequence::default_snapshot;
    use protocol::testkit::assert_missing_target_is_error;

    #[semio_framework_async_macros::async_test]
    async fn disconnect_family_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &disconnect_steps("missing".into()));
    }
}
//#endregion 🧪️MutationLaws
