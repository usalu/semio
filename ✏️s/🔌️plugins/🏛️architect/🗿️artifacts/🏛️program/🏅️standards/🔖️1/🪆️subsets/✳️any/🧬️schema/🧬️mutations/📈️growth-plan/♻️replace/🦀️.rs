//! 🦠️ ProgramSnapshot mutation — `replace-growth-plan` leaf (replace). Split from the
//! pre-migration `📈growth` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::GrowthPlan;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🔁️ Whole-value swap of one growth plan row's non-identity content, addressed by
/// `growth_plan.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ReplaceGrowthPlan {
    pub growth_plan: GrowthPlan,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceGrowthPlan {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "growth-plan", kind: "replace-growth-plan", record: "ReplacedGrowthPlan" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace growth plan \"{}\"", self.growth_plan.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.growth_plan.header.id.0.clone()]
    }
}
