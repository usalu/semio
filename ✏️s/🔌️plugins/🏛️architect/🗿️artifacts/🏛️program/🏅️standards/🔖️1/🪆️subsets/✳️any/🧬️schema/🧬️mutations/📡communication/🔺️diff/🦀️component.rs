//! 🔺️ Sparse diff construction for the `communication` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateCommunicationRequirement, DeleteCommunicationRequirement, RenameCommunicationRequirement, ReplaceCommunicationRequirement};
use crate::artifacts::program::diff::{ProgramCommunicationDelta, ProgramCommunicationPatchEntry};
use crate::artifacts::program::registers::CommunicationRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.communication` on apply.
pub fn diff_create(payload: &CreateCommunicationRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { communication: Some(ProgramCommunicationDelta { added: vec![payload.communication_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteCommunicationRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { communication: Some(ProgramCommunicationDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameCommunicationRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = CommunicationRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { communication: Some(ProgramCommunicationDelta { patched: vec![ProgramCommunicationPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceCommunicationRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.communication.iter().find(|row| row.header.id == payload.communication_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.communication_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { communication: Some(ProgramCommunicationDelta { patched: vec![ProgramCommunicationPatchEntry { id: payload.communication_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
