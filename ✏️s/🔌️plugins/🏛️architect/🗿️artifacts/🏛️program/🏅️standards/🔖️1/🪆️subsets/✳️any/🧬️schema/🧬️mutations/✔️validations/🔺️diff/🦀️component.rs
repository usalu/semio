//! 🔺️ Sparse diff construction for the `validations` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateValidationRecord, DeleteValidationRecord, RenameValidationRecord, ReplaceValidationRecord};
use crate::artifacts::program::diff::{ProgramValidationsDelta, ProgramValidationsPatchEntry};
use crate::artifacts::program::registers::ValidationRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.validations` on apply.
pub fn diff_create(payload: &CreateValidationRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { validations: Some(ProgramValidationsDelta { added: vec![payload.validation_record.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteValidationRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { validations: Some(ProgramValidationsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameValidationRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ValidationRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { validations: Some(ProgramValidationsDelta { patched: vec![ProgramValidationsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceValidationRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.validations.iter().find(|row| row.header.id == payload.validation_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.validation_record).expect("diff_patch always produces a full patch");
    ProgramDiff { validations: Some(ProgramValidationsDelta { patched: vec![ProgramValidationsPatchEntry { id: payload.validation_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
