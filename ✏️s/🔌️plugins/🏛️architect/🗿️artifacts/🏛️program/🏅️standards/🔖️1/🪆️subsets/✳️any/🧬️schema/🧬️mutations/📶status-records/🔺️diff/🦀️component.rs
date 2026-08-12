//! 🔺️ Sparse diff construction for the `status_records` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateStatusRecord, DeleteStatusRecord, RenameStatusRecord, ReplaceStatusRecord};
use crate::artifacts::program::diff::{ProgramStatusRecordsDelta, ProgramStatusRecordsPatchEntry};
use crate::artifacts::program::registers::StatusRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.status_records` on apply.
pub fn diff_create(payload: &CreateStatusRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { status_records: Some(ProgramStatusRecordsDelta { added: vec![payload.status_record.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteStatusRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { status_records: Some(ProgramStatusRecordsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameStatusRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = StatusRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { status_records: Some(ProgramStatusRecordsDelta { patched: vec![ProgramStatusRecordsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceStatusRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.status_records.iter().find(|row| row.header.id == payload.status_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.status_record).expect("diff_patch always produces a full patch");
    ProgramDiff { status_records: Some(ProgramStatusRecordsDelta { patched: vec![ProgramStatusRecordsPatchEntry { id: payload.status_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
