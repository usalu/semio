//! 🔺️ Sparse diff construction for the `resilience` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateResilienceRequirement, DeleteResilienceRequirement, RenameResilienceRequirement, ReplaceResilienceRequirement};
use crate::artifacts::program::diff::{ProgramResilienceDelta, ProgramResiliencePatchEntry};
use crate::artifacts::program::registers::ResilienceRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.resilience` on apply.
pub fn diff_create(payload: &CreateResilienceRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { resilience: Some(ProgramResilienceDelta { added: vec![payload.resilience_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteResilienceRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { resilience: Some(ProgramResilienceDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameResilienceRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ResilienceRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { resilience: Some(ProgramResilienceDelta { patched: vec![ProgramResiliencePatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceResilienceRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.resilience.iter().find(|row| row.header.id == payload.resilience_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.resilience_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { resilience: Some(ProgramResilienceDelta { patched: vec![ProgramResiliencePatchEntry { id: payload.resilience_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
