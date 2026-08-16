//! 🔺️ Sparse diff construction for the `rename-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📌requirements` per Wave C.

use super::mutation::RenameRequirement;
use crate::artifacts::program::diff::{ProgramRequirementsDelta, ProgramRequirementsPatchEntry};
use crate::artifacts::program::registers::RequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = RequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { requirements: Some(ProgramRequirementsDelta { patched: vec![ProgramRequirementsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
