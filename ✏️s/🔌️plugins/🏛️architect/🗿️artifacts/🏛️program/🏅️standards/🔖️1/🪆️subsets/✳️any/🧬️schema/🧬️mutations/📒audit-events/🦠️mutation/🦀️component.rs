//! 🦠️ ProgramSnapshot mutation — `audit_events` leaf: create/delete/rename/replace audit event rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `AuditEvent` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::AuditEvent;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateAuditEvent
/// 🌱️ Brings a new audit event row into existence in `program.audit_events`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAuditEvent {
    pub audit_event: AuditEvent,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateAuditEvent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "audit-event", kind: "create-audit-event", record: "CreatedAuditEvent" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create audit event \"{}\"", self.audit_event.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.audit_event.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateAuditEvent

//#region 🔖️DeleteAuditEvent
/// 🗑️ Removes a audit event row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAuditEvent {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteAuditEvent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "audit-event", kind: "delete-audit-event", record: "DeletedAuditEvent" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete audit event \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteAuditEvent

//#region 🔖️RenameAuditEvent
/// ✏️ Sets the identity `name` field of one audit event row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameAuditEvent {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameAuditEvent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "audit-event", kind: "rename-audit-event", record: "RenamedAuditEvent" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename audit event to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameAuditEvent

//#region 🔖️ReplaceAuditEvent
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
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace audit event \"{}\"", self.audit_event.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.audit_event.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceAuditEvent
