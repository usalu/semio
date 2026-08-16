//! 🦠️ ProgramSnapshot mutation — `create-priority-record` leaf (create). Split from the
//! pre-migration `⭐priorities` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::PriorityRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new priority record row into existence in `program.priorities`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePriorityRecord {
    pub priority_record: PriorityRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreatePriorityRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "priority-record", kind: "create-priority-record", record: "CreatedPriorityRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create priority record \"{}\"", self.priority_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.priority_record.header.id.0.clone()]
    }
}
