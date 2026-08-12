//! 🔺️ Sparse diff construction for the `priorities` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreatePriorityRecord, DeletePriorityRecord, RenamePriorityRecord, ReplacePriorityRecord};
use crate::artifacts::program::diff::{ProgramPrioritiesDelta, ProgramPrioritiesPatchEntry};
use crate::artifacts::program::registers::PriorityRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.priorities` on apply.
pub fn diff_create(payload: &CreatePriorityRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { priorities: Some(ProgramPrioritiesDelta { added: vec![payload.priority_record.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeletePriorityRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { priorities: Some(ProgramPrioritiesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenamePriorityRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = PriorityRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { priorities: Some(ProgramPrioritiesDelta { patched: vec![ProgramPrioritiesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplacePriorityRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.priorities.iter().find(|row| row.header.id == payload.priority_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.priority_record).expect("diff_patch always produces a full patch");
    ProgramDiff { priorities: Some(ProgramPrioritiesDelta { patched: vec![ProgramPrioritiesPatchEntry { id: payload.priority_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
