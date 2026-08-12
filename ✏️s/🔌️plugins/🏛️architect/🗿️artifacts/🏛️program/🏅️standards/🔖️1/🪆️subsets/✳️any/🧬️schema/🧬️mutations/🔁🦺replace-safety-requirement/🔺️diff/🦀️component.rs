//! 🔺️ Sparse diff construction for the `replace-safety-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🦺safety` per Wave C.

use super::mutation::ReplaceSafetyRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSafetyDelta, ProgramSafetyPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceSafetyRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.safety.iter().find(|row| row.header.id == payload.safety_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.safety_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { safety: Some(ProgramSafetyDelta { patched: vec![ProgramSafetyPatchEntry { id: payload.safety_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
