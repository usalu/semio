//! 🧬️ Sequence mutation — `DuplicateStep`: copies a step to a new identity/position.
use crate::diff::SequenceDiff;
use crate::mutations::SequenceMutation;
use crate::SequenceSnapshot;

//#region 🔖️Mutation
/// 🧬️ `duplicate-step` payload — source step id, the copy's new id, and its FINAL position.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "duplicate-step")]
pub struct DuplicateStep {
    pub source_id: String,
    pub new_id: String,
    pub x: f64,
    pub y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn duplicate_step(source_id: String, new_id: String, x: f64, y: f64) -> SequenceMutation {
    SequenceMutation::DuplicateStep(DuplicateStep { source_id, new_id, x, y })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for DuplicateStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "duplicate", entity: "step", kind: "duplicate-step", record: "DuplicatedStep" };

    fn diff(&self, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Duplicate step \"{}\" as \"{}\"", self.source_id, self.new_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.new_id.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️MutationLaws
#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-law/🦀️.rs"]
mod mutation_law_tests;
//#endregion 🧪️MutationLaws
