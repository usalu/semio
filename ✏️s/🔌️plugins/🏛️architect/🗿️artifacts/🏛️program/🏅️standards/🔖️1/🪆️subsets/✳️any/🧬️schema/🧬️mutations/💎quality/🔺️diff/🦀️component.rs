//! 🔺️ Sparse diff construction for the `quality` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateQualityRecord, DeleteQualityRecord, RenameQualityRecord, ReplaceQualityRecord};
use crate::artifacts::program::diff::{ProgramQualityDelta, ProgramQualityPatchEntry};
use crate::artifacts::program::registers::QualityRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.quality` on apply.
pub fn diff_create(payload: &CreateQualityRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { quality: Some(ProgramQualityDelta { added: vec![payload.quality_record.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteQualityRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { quality: Some(ProgramQualityDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameQualityRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = QualityRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { quality: Some(ProgramQualityDelta { patched: vec![ProgramQualityPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceQualityRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.quality.iter().find(|row| row.header.id == payload.quality_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.quality_record).expect("diff_patch always produces a full patch");
    ProgramDiff { quality: Some(ProgramQualityDelta { patched: vec![ProgramQualityPatchEntry { id: payload.quality_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
