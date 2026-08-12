//! 🔺️ Sparse diff construction for the `rename-safety-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🦺safety` per Wave C.

use super::mutation::RenameSafetyRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSafetyDelta, ProgramSafetyPatchEntry};
use crate::artifacts::program::registers::SafetyRequirementPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameSafetyRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = SafetyRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { safety: Some(ProgramSafetyDelta { patched: vec![ProgramSafetyPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
