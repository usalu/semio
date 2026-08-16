//! 🦠️ ProgramSnapshot mutation — `create-performance-criterion` leaf (create). Split from the
//! pre-migration `📊performance` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::PerformanceCriterion;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new performance criterion row into existence in `program.performance`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePerformanceCriterion {
    pub performance_criterion: PerformanceCriterion,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreatePerformanceCriterion {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "performance-criterion", kind: "create-performance-criterion", record: "CreatedPerformanceCriterion" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create performance criterion \"{}\"", self.performance_criterion.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.performance_criterion.header.id.0.clone()]
    }
}
