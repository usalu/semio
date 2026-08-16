//! 🔺️ Sparse diff construction for the `create-environmental-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌿environmental` per Wave C.

use super::mutation::CreateEnvironmentalRequirement;
use crate::artifacts::program::diff::ProgramEnvironmentalDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.environmental` on apply.
pub fn diff(payload: &CreateEnvironmentalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { environmental: Some(ProgramEnvironmentalDelta { added: vec![payload.environmental_requirement.clone()], ..Default::default() }), ..Default::default() }
}
