//! 🔗 Sequence mutation — `ConnectSteps`: creates a flow edge relationship between two steps.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::schema::operations::{SequenceDetectedMutation, SequenceDetectionContext};
use crate::artifacts::sequence::SequenceSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔗 `connect-steps` payload — edge `id` plus both endpoint step ids.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
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

//#region 🔎️Detection
/// 🔎️ Detects this leaf's contribution to a before/after sequence plan.
pub fn detect(context: &SequenceDetectionContext<'_>) -> Vec<SequenceDetectedMutation> {
    context
        .after
        .edges
        .iter()
        .enumerate()
        .filter(|(_, edge)| context.before_edges.get(edge.id.as_str()).map_or(true, |before| before.from != edge.from || before.to != edge.to))
        .map(|(index, edge)| SequenceDetectedMutation { order: (3, index, 1), mutation: SequenceMutation::ConnectSteps(ConnectSteps { id: edge.id.clone(), from: edge.from.clone(), to: edge.to.clone() }) })
        .collect()
}
//#endregion 🔎️Detection

//#region 🧪️MutationLaws
#[cfg(test)]
mod mutation_law_tests {
    use super::*;
    use crate::artifacts::sequence::default_snapshot;
    use protocol::{
        testkit::{assert_fatal_never_applies, assert_missing_target_is_error},
        Mutation,
    };

    #[semio_framework_async_macros::async_test]
    async fn connect_family_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &connect_steps("edge-99".into(), "missing".into(), "step-2".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_family_fatal_never_applies() {
        let base = default_snapshot();
        let outcome = connect_steps("edge-99".into(), "step-1".into(), "step-1".into()).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
        assert_fatal_never_applies(&outcome);
    }
}
//#endregion 🧪️MutationLaws
