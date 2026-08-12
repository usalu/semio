//! 🔺️ Sparse diff construction for the `rename-environmental-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌿environmental` per Wave C.

use super::mutation::RenameEnvironmentalRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramEnvironmentalDelta, ProgramEnvironmentalPatchEntry};
use crate::artifacts::program::registers::EnvironmentalRequirementPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameEnvironmentalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = EnvironmentalRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { environmental: Some(ProgramEnvironmentalDelta { patched: vec![ProgramEnvironmentalPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
