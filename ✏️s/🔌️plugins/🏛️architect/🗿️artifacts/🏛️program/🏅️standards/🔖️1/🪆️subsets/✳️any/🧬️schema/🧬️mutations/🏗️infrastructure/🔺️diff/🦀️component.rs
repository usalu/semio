//! 🔺️ Sparse diff construction for the `infrastructure` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateInfrastructureRequirement, DeleteInfrastructureRequirement, RenameInfrastructureRequirement, ReplaceInfrastructureRequirement};
use crate::artifacts::program::diff::{ProgramInfrastructureDelta, ProgramInfrastructurePatchEntry};
use crate::artifacts::program::registers::InfrastructureRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.infrastructure` on apply.
pub fn diff_create(payload: &CreateInfrastructureRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { infrastructure: Some(ProgramInfrastructureDelta { added: vec![payload.infrastructure_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteInfrastructureRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { infrastructure: Some(ProgramInfrastructureDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameInfrastructureRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = InfrastructureRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { infrastructure: Some(ProgramInfrastructureDelta { patched: vec![ProgramInfrastructurePatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceInfrastructureRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.infrastructure.iter().find(|row| row.header.id == payload.infrastructure_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.infrastructure_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { infrastructure: Some(ProgramInfrastructureDelta { patched: vec![ProgramInfrastructurePatchEntry { id: payload.infrastructure_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
