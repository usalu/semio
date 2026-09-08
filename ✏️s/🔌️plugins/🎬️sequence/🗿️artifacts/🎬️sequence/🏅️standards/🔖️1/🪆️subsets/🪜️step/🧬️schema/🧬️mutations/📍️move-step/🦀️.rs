//! ↔️ Sequence mutation — `MoveStep`: absolute spatial reposition of a canvas node.
use crate::diff::SequenceDiff;
use crate::mutations::SequenceMutation;
use crate::schema::operations::{SequenceDetectedMutation, SequenceDetectionContext};
use crate::SequenceSnapshot;

//#region 🔖️Mutation
/// ↔️ `move-step` payload — FINAL-state absolute `(x, y)`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "move-step")]
pub struct MoveStep {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_step(id: String, x: f64, y: f64) -> SequenceMutation {
    SequenceMutation::MoveStep(MoveStep { id, x, y })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for MoveStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "step", kind: "move-step", record: "MovedStep" };

    fn diff(&self, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move step \"{}\" to ({}, {})", self.id, self.x, self.y)
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
        .steps
        .iter()
        .enumerate()
        .filter_map(|(index, step)| {
            let before = context.before_steps.get(step.id.as_str())?;
            (before.x != step.x || before.y != step.y).then(|| SequenceDetectedMutation { order: (1, index, 0), mutation: SequenceMutation::MoveStep(MoveStep { id: step.id.clone(), x: step.x, y: step.y }) })
        })
        .collect()
}
//#endregion 🔎️Detection

//#region 🧪️MutationLaws
#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-law/🦀️.rs"]
mod mutation_law_tests;
//#endregion 🧪️MutationLaws
