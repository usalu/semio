//! 🔺️ Sparse diff construction for the `rename-sustainability-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♻️sustainability` per Wave C.

use super::mutation::RenameSustainabilityRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSustainabilityDelta, ProgramSustainabilityPatchEntry};
use crate::artifacts::program::registers::SustainabilityRequirementPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameSustainabilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = SustainabilityRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { sustainability: Some(ProgramSustainabilityDelta { patched: vec![ProgramSustainabilityPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
