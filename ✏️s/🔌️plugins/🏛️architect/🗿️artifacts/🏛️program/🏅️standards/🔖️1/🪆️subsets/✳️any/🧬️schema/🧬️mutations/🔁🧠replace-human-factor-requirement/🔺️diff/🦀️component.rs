//! 🔺️ Sparse diff construction for the `replace-human-factor-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧠human-factors` per Wave C.

use super::mutation::ReplaceHumanFactorRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramHumanFactorsDelta, ProgramHumanFactorsPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceHumanFactorRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.human_factors.iter().find(|row| row.header.id == payload.human_factor_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.human_factor_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { human_factors: Some(ProgramHumanFactorsDelta { patched: vec![ProgramHumanFactorsPatchEntry { id: payload.human_factor_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
