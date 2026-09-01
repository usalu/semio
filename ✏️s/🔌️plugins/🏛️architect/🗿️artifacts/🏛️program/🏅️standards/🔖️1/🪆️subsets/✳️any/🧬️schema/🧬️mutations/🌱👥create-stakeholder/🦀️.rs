//! 🦠️ ProgramSnapshot mutation — `create-stakeholder` leaf (create). Split from the
//! pre-migration `👥stakeholders` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::Stakeholder;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new stakeholder row into existence in `program.stakeholders`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct CreateStakeholder {
    pub stakeholder: Stakeholder,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateStakeholder {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "stakeholder", kind: "create-stakeholder", record: "CreatedStakeholder" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create stakeholder \"{}\"", self.stakeholder.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.stakeholder.header.id.0.clone()]
    }
}
