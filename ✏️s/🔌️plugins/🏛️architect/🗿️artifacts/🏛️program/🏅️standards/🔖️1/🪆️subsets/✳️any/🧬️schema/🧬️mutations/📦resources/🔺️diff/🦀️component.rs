//! 🔺️ Sparse diff construction for the `resources` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateResource, DeleteResource, RenameResource, ReplaceResource};
use crate::artifacts::program::diff::{ProgramResourcesDelta, ProgramResourcesPatchEntry};
use crate::artifacts::program::registers::ResourcePatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.resources` on apply.
pub fn diff_create(payload: &CreateResource, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { resources: Some(ProgramResourcesDelta { added: vec![payload.resource.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteResource, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { resources: Some(ProgramResourcesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameResource, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ResourcePatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { resources: Some(ProgramResourcesDelta { patched: vec![ProgramResourcesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceResource, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.resources.iter().find(|row| row.header.id == payload.resource.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.resource).expect("diff_patch always produces a full patch");
    ProgramDiff { resources: Some(ProgramResourcesDelta { patched: vec![ProgramResourcesPatchEntry { id: payload.resource.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
