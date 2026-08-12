//! 🔺️ Sparse diff construction for the `approvals` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateApprovalRecord, DeleteApprovalRecord, RenameApprovalRecord, ReplaceApprovalRecord};
use crate::artifacts::program::diff::{ProgramApprovalsDelta, ProgramApprovalsPatchEntry};
use crate::artifacts::program::registers::ApprovalRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.approvals` on apply.
pub fn diff_create(payload: &CreateApprovalRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { approvals: Some(ProgramApprovalsDelta { added: vec![payload.approval_record.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteApprovalRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { approvals: Some(ProgramApprovalsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameApprovalRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ApprovalRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { approvals: Some(ProgramApprovalsDelta { patched: vec![ProgramApprovalsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceApprovalRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.approvals.iter().find(|row| row.header.id == payload.approval_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.approval_record).expect("diff_patch always produces a full patch");
    ProgramDiff { approvals: Some(ProgramApprovalsDelta { patched: vec![ProgramApprovalsPatchEntry { id: payload.approval_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
