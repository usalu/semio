//! 🔗 Sequence mutation — `ConnectSteps`: creates a flow edge relationship between two steps.
use crate::diff::SequenceDiff;
use crate::mutations::SequenceMutation;
use crate::schema::operations::{SequenceDetectedMutation, SequenceDetectionContext};
use crate::SequenceSnapshot;

//#region 🔖️Mutation
/// 🔗 `connect-steps` payload — edge `id` plus both endpoint step ids.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "connect-steps")]
pub struct ConnectSteps {
    pub id: String,
    pub from: String,
    pub to: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_steps(id: String, from: String, to: String) -> SequenceMutation {
    SequenceMutation::ConnectSteps(ConnectSteps { id, from, to })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for ConnectSteps {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "steps", kind: "connect-steps", record: "ConnectedSteps" };

    fn diff(&self, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Connect step \"{}\" to \"{}\"", self.from, self.to)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🔎️Detection
/// 🔎️ Detects this leaf's contribution to a before/after sequence plan.
pub fn detect(context: &SequenceDetectionContext<'_>) -> Vec<SequenceDetectedMutation> {
    context
        .after
        .edges
        .iter()
        .enumerate()
        .filter(|(_, edge)| context.before_edges.get(edge.id.as_str()).is_none_or(|before| before.from != edge.from || before.to != edge.to))
        .map(|(index, edge)| SequenceDetectedMutation { order: (3, index, 1), mutation: SequenceMutation::ConnectSteps(ConnectSteps { id: edge.id.clone(), from: edge.from.clone(), to: edge.to.clone() }) })
        .collect()
}
//#endregion 🔎️Detection

//#region 🧪️MutationLaws
#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-law/🦀️.rs"]
mod mutation_law_tests;
//#endregion 🧪️MutationLaws
