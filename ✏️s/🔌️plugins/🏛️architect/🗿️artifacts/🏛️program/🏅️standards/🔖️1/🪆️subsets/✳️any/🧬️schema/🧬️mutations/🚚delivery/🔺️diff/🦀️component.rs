//! 🔺️ Sparse diff construction for the `delivery` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateDeliveryConstraint, DeleteDeliveryConstraint, RenameDeliveryConstraint, ReplaceDeliveryConstraint};
use crate::artifacts::program::diff::{ProgramDeliveryDelta, ProgramDeliveryPatchEntry};
use crate::artifacts::program::registers::DeliveryConstraintPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.delivery` on apply.
pub fn diff_create(payload: &CreateDeliveryConstraint, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { delivery: Some(ProgramDeliveryDelta { added: vec![payload.delivery_constraint.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteDeliveryConstraint, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { delivery: Some(ProgramDeliveryDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameDeliveryConstraint, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = DeliveryConstraintPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { delivery: Some(ProgramDeliveryDelta { patched: vec![ProgramDeliveryPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceDeliveryConstraint, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.delivery.iter().find(|row| row.header.id == payload.delivery_constraint.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.delivery_constraint).expect("diff_patch always produces a full patch");
    ProgramDiff { delivery: Some(ProgramDeliveryDelta { patched: vec![ProgramDeliveryPatchEntry { id: payload.delivery_constraint.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
