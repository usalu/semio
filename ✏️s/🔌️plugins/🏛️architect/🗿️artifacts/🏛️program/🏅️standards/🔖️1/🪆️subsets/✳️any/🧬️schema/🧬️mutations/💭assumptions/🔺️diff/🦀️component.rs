//! 🔺️ Sparse diff construction for the `assumptions` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateAssumption, DeleteAssumption, RenameAssumption, ReplaceAssumption};
use crate::artifacts::program::diff::{ProgramAssumptionsDelta, ProgramAssumptionsPatchEntry};
use crate::artifacts::program::registers::AssumptionPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.assumptions` on apply.
pub fn diff_create(payload: &CreateAssumption, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { assumptions: Some(ProgramAssumptionsDelta { added: vec![payload.assumption.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteAssumption, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { assumptions: Some(ProgramAssumptionsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameAssumption, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = AssumptionPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { assumptions: Some(ProgramAssumptionsDelta { patched: vec![ProgramAssumptionsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceAssumption, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.assumptions.iter().find(|row| row.header.id == payload.assumption.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.assumption).expect("diff_patch always produces a full patch");
    ProgramDiff { assumptions: Some(ProgramAssumptionsDelta { patched: vec![ProgramAssumptionsPatchEntry { id: payload.assumption.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
