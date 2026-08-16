//! 🔺️ Sparse diff construction for the `replace-quantity-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔢quantities` per Wave C.

use super::mutation::ReplaceQuantityRequirement;
use crate::artifacts::program::diff::{ProgramQuantitiesDelta, ProgramQuantitiesPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceQuantityRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.quantities.iter().find(|row| row.header.id == payload.quantity_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.quantity_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { quantities: Some(ProgramQuantitiesDelta { patched: vec![ProgramQuantitiesPatchEntry { id: payload.quantity_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
