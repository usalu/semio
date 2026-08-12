//! 🔺️ Sparse diff construction for the `replace-sustainability-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♻️sustainability` per Wave C.

use super::mutation::ReplaceSustainabilityRequirement;
use protocol::Patchable;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSustainabilityDelta, ProgramSustainabilityPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceSustainabilityRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.sustainability.iter().find(|row| row.header.id == payload.sustainability_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.sustainability_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { sustainability: Some(ProgramSustainabilityDelta { patched: vec![ProgramSustainabilityPatchEntry { id: payload.sustainability_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
