//! 🦠️ ProgramSnapshot mutation — `create-collaboration-record` leaf (create). Split from the
//! pre-migration `🤝collaboration` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::CollaborationRecord;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new collaboration record row into existence in `program.collaboration`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCollaborationRecord {
    pub collaboration_record: CollaborationRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateCollaborationRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "collaboration-record", kind: "create-collaboration-record", record: "CreatedCollaborationRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create collaboration record \"{}\"", self.collaboration_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.collaboration_record.header.id.0.clone()]
    }
}
