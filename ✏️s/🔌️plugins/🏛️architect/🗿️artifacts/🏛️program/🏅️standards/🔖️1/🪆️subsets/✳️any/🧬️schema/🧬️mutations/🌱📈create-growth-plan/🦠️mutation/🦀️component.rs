//! 🦠️ ProgramSnapshot mutation — `create-growth-plan` leaf (create). Split from the
//! pre-migration `📈growth` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::GrowthPlan;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new growth plan row into existence in `program.growth`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGrowthPlan {
    pub growth_plan: GrowthPlan,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateGrowthPlan {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "growth-plan", kind: "create-growth-plan", record: "CreatedGrowthPlan" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create growth plan \"{}\"", self.growth_plan.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.growth_plan.header.id.0.clone()]
    }
}
