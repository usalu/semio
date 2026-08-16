//! 🦠️ ProgramSnapshot mutation — `create-change-record` leaf (create). Split from the
//! pre-migration `📝changes` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::ChangeRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new change record row into existence in `program.changes`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateChangeRecord {
    pub change_record: ChangeRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateChangeRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "change-record", kind: "create-change-record", record: "CreatedChangeRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create change record \"{}\"", self.change_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.change_record.header.id.0.clone()]
    }
}
