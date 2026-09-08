//! ✂️ Sequence mutation — `DisconnectSteps`: removes a flow edge relationship.
use crate::diff::SequenceDiff;
use crate::mutations::SequenceMutation;
use crate::schema::operations::{SequenceDetectedMutation, SequenceDetectionContext};
use crate::SequenceSnapshot;

//#region 🔖️Mutation
/// ✂️ `disconnect-steps` payload — edge id.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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

    fn diff(&self, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
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
#[path = "🧪️tests/🔬️mutation-law/🦀️.rs"]
mod mutation_law_tests;
//#endregion 🧪️MutationLaws
