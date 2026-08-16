//! 🦠️ ProgramSnapshot mutation — `replace-schedule-requirement` leaf (replace). Split from the
//! pre-migration `📅schedules` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::ScheduleRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one schedule requirement row's non-identity content, addressed by
/// `schedule_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceScheduleRequirement {
    pub schedule_requirement: ScheduleRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceScheduleRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "schedule-requirement", kind: "replace-schedule-requirement", record: "ReplacedScheduleRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace schedule requirement \"{}\"", self.schedule_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.schedule_requirement.header.id.0.clone()]
    }
}
