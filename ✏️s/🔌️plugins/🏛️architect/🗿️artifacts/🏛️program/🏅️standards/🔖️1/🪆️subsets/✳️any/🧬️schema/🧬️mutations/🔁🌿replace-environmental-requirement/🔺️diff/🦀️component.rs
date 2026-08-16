//! 🔺️ Sparse diff construction for the `replace-environmental-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌿environmental` per Wave C.

use super::mutation::ReplaceEnvironmentalRequirement;
use crate::artifacts::program::diff::{ProgramEnvironmentalDelta, ProgramEnvironmentalPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceEnvironmentalRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.environmental.iter().find(|row| row.header.id == payload.environmental_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.environmental_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { environmental: Some(ProgramEnvironmentalDelta { patched: vec![ProgramEnvironmentalPatchEntry { id: payload.environmental_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
