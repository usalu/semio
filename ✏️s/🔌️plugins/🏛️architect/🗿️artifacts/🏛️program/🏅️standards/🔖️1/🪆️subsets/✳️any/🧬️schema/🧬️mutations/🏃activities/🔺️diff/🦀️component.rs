//! 🔺️ Sparse diff construction for the `activities` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateActivity, DeleteActivity, RenameActivity, ReplaceActivity};
use crate::artifacts::program::diff::{ProgramActivitiesDelta, ProgramActivitiesPatchEntry};
use crate::artifacts::program::registers::ActivityPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.activities` on apply.
pub fn diff_create(payload: &CreateActivity, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { activities: Some(ProgramActivitiesDelta { added: vec![payload.activity.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteActivity, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { activities: Some(ProgramActivitiesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameActivity, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ActivityPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { activities: Some(ProgramActivitiesDelta { patched: vec![ProgramActivitiesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceActivity, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.activities.iter().find(|row| row.header.id == payload.activity.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.activity).expect("diff_patch always produces a full patch");
    ProgramDiff { activities: Some(ProgramActivitiesDelta { patched: vec![ProgramActivitiesPatchEntry { id: payload.activity.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
