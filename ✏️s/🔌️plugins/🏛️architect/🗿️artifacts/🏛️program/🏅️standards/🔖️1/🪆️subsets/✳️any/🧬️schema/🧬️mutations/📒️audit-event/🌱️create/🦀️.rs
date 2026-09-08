//! 🦠️ ProgramSnapshot mutation — `create-audit-event` leaf (create). Split from the
//! pre-migration `📒audit-events` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::registers::AuditEvent;
use crate::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🌱️ Brings a new audit event row into existence in `program.audit_events`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CreateAuditEvent {
    pub audit_event: AuditEvent,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateAuditEvent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "audit-event", kind: "create-audit-event", record: "CreatedAuditEvent" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create audit event \"{}\"", self.audit_event.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.audit_event.header.id.0.clone()]
    }
}
