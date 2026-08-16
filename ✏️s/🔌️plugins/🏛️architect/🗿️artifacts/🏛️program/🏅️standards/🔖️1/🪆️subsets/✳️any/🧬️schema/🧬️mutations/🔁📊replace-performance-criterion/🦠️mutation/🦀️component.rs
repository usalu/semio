//! 🦠️ ProgramSnapshot mutation — `replace-performance-criterion` leaf (replace). Split from the
//! pre-migration `📊performance` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::PerformanceCriterion;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one performance criterion row's non-identity content, addressed by
/// `performance_criterion.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplacePerformanceCriterion {
    pub performance_criterion: PerformanceCriterion,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplacePerformanceCriterion {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "performance-criterion", kind: "replace-performance-criterion", record: "ReplacedPerformanceCriterion" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace performance criterion \"{}\"", self.performance_criterion.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.performance_criterion.header.id.0.clone()]
    }
}
