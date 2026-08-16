//! 🦠️ ProgramSnapshot mutation — `replace-cost-requirement` leaf (replace). Split from the
//! pre-migration `💰costs` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::CostRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one cost requirement row's non-identity content, addressed by
/// `cost_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceCostRequirement {
    pub cost_requirement: CostRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceCostRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "cost-requirement", kind: "replace-cost-requirement", record: "ReplacedCostRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace cost requirement \"{}\"", self.cost_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.cost_requirement.header.id.0.clone()]
    }
}
