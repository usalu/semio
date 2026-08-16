//! 🦠️ ProgramSnapshot mutation — `replace-approval-record` leaf (replace). Split from the
//! pre-migration `👍approvals` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::ApprovalRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one approval record row's non-identity content, addressed by
/// `approval_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceApprovalRecord {
    pub approval_record: ApprovalRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceApprovalRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "approval-record", kind: "replace-approval-record", record: "ReplacedApprovalRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace approval record \"{}\"", self.approval_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.approval_record.header.id.0.clone()]
    }
}
