//! 🔺️ Sparse diff construction for the `schedules` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateScheduleRequirement, DeleteScheduleRequirement, RenameScheduleRequirement, ReplaceScheduleRequirement};
use crate::artifacts::program::diff::{ProgramSchedulesDelta, ProgramSchedulesPatchEntry};
use crate::artifacts::program::registers::ScheduleRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.schedules` on apply.
pub fn diff_create(payload: &CreateScheduleRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { schedules: Some(ProgramSchedulesDelta { added: vec![payload.schedule_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteScheduleRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { schedules: Some(ProgramSchedulesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameScheduleRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ScheduleRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { schedules: Some(ProgramSchedulesDelta { patched: vec![ProgramSchedulesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceScheduleRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.schedules.iter().find(|row| row.header.id == payload.schedule_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.schedule_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { schedules: Some(ProgramSchedulesDelta { patched: vec![ProgramSchedulesPatchEntry { id: payload.schedule_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
