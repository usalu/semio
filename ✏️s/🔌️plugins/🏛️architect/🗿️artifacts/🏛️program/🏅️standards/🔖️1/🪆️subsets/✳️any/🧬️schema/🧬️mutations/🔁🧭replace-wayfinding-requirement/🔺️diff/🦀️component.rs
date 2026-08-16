//! 🔺️ Sparse diff construction for the `replace-wayfinding-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧭wayfinding` per Wave C.

use super::mutation::ReplaceWayfindingRequirement;
use crate::artifacts::program::diff::{ProgramWayfindingDelta, ProgramWayfindingPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceWayfindingRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.wayfinding.iter().find(|row| row.header.id == payload.wayfinding_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.wayfinding_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { wayfinding: Some(ProgramWayfindingDelta { patched: vec![ProgramWayfindingPatchEntry { id: payload.wayfinding_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
