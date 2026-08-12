//! 🔺️ Sparse diff construction for the `operations` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateOperationalRequirement, DeleteOperationalRequirement, RenameOperationalRequirement, ReplaceOperationalRequirement};
use crate::artifacts::program::diff::{ProgramOperationsDelta, ProgramOperationsPatchEntry};
use crate::artifacts::program::registers::OperationalRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.operations` on apply.
pub fn diff_create(payload: &CreateOperationalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { operations: Some(ProgramOperationsDelta { added: vec![payload.operational_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteOperationalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { operations: Some(ProgramOperationsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameOperationalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = OperationalRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { operations: Some(ProgramOperationsDelta { patched: vec![ProgramOperationsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceOperationalRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.operations.iter().find(|row| row.header.id == payload.operational_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.operational_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { operations: Some(ProgramOperationsDelta { patched: vec![ProgramOperationsPatchEntry { id: payload.operational_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
