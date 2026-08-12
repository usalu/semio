//! 🦠️ ProgramSnapshot mutation — `replace-status-record` leaf (replace). Split from the
//! pre-migration `📶status-records` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::StatusRecord;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one status record row's non-identity content, addressed by
/// `status_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceStatusRecord {
    pub status_record: StatusRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceStatusRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "status-record", kind: "replace-status-record", record: "ReplacedStatusRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace status record \"{}\"", self.status_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.status_record.header.id.0.clone()]
    }
}
