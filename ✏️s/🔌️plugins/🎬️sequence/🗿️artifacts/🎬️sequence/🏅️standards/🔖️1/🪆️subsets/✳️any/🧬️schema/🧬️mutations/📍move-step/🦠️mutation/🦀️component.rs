//! ↔️ Sequence mutation — `MoveStep`: absolute spatial reposition of a canvas node.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::SequenceSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ↔️ `move-step` payload — FINAL-state absolute `(x, y)`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-step")]
pub struct MoveStep {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn move_step(id: String, x: f64, y: f64) -> SequenceMutation {
    SequenceMutation::MoveStep(MoveStep { id, x, y })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for MoveStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "step", kind: "move-step", record: "MovedStep" };

    async fn diff(&self, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Move step \"{}\" to ({}, {})", self.id, self.x, self.y)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
