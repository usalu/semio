//! 🔺️ Sparse diff construction for the `replace-flexibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧩flexibility` per Wave C.

use super::mutation::ReplaceFlexibilityRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramFlexibilityDelta, ProgramFlexibilityPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceFlexibilityRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.flexibility.iter().find(|row| row.header.id == payload.flexibility_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.flexibility_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { flexibility: Some(ProgramFlexibilityDelta { patched: vec![ProgramFlexibilityPatchEntry { id: payload.flexibility_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
