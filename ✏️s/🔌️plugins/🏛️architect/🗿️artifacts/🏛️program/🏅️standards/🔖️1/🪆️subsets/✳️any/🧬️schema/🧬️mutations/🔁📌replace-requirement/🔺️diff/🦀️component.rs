//! 🔺️ Sparse diff construction for the `replace-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📌requirements` per Wave C.

use super::mutation::ReplaceRequirement;
use protocol::Patchable;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramRequirementsDelta, ProgramRequirementsPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.requirements.iter().find(|row| row.header.id == payload.requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { requirements: Some(ProgramRequirementsDelta { patched: vec![ProgramRequirementsPatchEntry { id: payload.requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
