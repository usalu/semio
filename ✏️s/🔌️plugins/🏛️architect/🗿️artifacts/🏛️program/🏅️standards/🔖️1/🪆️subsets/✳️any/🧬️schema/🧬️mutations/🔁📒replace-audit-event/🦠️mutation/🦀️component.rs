//! 🦠️ ProgramSnapshot mutation — `replace-audit-event` leaf (replace). Split from the
//! pre-migration `📒audit-events` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::AuditEvent;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one audit event row's non-identity content, addressed by
/// `audit_event.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceAuditEvent {
    pub audit_event: AuditEvent,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceAuditEvent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "audit-event", kind: "replace-audit-event", record: "ReplacedAuditEvent" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace audit event \"{}\"", self.audit_event.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.audit_event.header.id.0.clone()]
    }
}
