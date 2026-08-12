//! 🔺️ Sparse diff construction for the `rename-human-factor-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧠human-factors` per Wave C.

use super::mutation::RenameHumanFactorRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramHumanFactorsDelta, ProgramHumanFactorsPatchEntry};
use crate::artifacts::program::registers::HumanFactorRequirementPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameHumanFactorRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = HumanFactorRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { human_factors: Some(ProgramHumanFactorsDelta { patched: vec![ProgramHumanFactorsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
