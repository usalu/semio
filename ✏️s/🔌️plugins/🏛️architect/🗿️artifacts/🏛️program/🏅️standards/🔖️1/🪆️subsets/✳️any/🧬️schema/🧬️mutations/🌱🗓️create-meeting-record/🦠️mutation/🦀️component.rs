//! 🦠️ ProgramSnapshot mutation — `create-meeting-record` leaf (create). Split from the
//! pre-migration `🗓️meetings` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::MeetingRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new meeting record row into existence in `program.meetings`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMeetingRecord {
    pub meeting_record: MeetingRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateMeetingRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "meeting-record", kind: "create-meeting-record", record: "CreatedMeetingRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create meeting record \"{}\"", self.meeting_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.meeting_record.header.id.0.clone()]
    }
}
