//! 🔺️ Sparse diff construction for the `audit_events` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateAuditEvent, DeleteAuditEvent, RenameAuditEvent, ReplaceAuditEvent};
use crate::artifacts::program::diff::{ProgramAuditEventsDelta, ProgramAuditEventsPatchEntry};
use crate::artifacts::program::registers::AuditEventPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.audit_events` on apply.
pub fn diff_create(payload: &CreateAuditEvent, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { audit_events: Some(ProgramAuditEventsDelta { added: vec![payload.audit_event.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteAuditEvent, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { audit_events: Some(ProgramAuditEventsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameAuditEvent, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = AuditEventPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { audit_events: Some(ProgramAuditEventsDelta { patched: vec![ProgramAuditEventsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceAuditEvent, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.audit_events.iter().find(|row| row.header.id == payload.audit_event.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.audit_event).expect("diff_patch always produces a full patch");
    ProgramDiff { audit_events: Some(ProgramAuditEventsDelta { patched: vec![ProgramAuditEventsPatchEntry { id: payload.audit_event.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
