//! 🦠️ ProgramSnapshot mutation — `create-cost-requirement` leaf (create). Split from the
//! pre-migration `💰costs` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::CostRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new cost requirement row into existence in `program.costs`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCostRequirement {
    pub cost_requirement: CostRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateCostRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "cost-requirement", kind: "create-cost-requirement", record: "CreatedCostRequirement" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create cost requirement \"{}\"", self.cost_requirement.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.cost_requirement.header.id.0.clone()]
    }
}
