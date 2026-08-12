//! 🔺️ Sparse diff construction for the `constraints` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateConstraintRecord, DeleteConstraintRecord, RenameConstraintRecord, ReplaceConstraintRecord};
use crate::artifacts::program::diff::{ProgramConstraintsDelta, ProgramConstraintsPatchEntry};
use crate::artifacts::program::registers::ConstraintRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.constraints` on apply.
pub fn diff_create(payload: &CreateConstraintRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { constraints: Some(ProgramConstraintsDelta { added: vec![payload.constraint_record.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteConstraintRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { constraints: Some(ProgramConstraintsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameConstraintRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ConstraintRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { constraints: Some(ProgramConstraintsDelta { patched: vec![ProgramConstraintsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceConstraintRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.constraints.iter().find(|row| row.header.id == payload.constraint_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.constraint_record).expect("diff_patch always produces a full patch");
    ProgramDiff { constraints: Some(ProgramConstraintsDelta { patched: vec![ProgramConstraintsPatchEntry { id: payload.constraint_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
