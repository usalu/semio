//! 🔺️ Sparse diff construction for the `create-safety-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🦺safety` per Wave C.

use super::mutation::CreateSafetyRequirement;
use crate::artifacts::program::diff::ProgramSafetyDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.safety` on apply.
pub fn diff(payload: &CreateSafetyRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { safety: Some(ProgramSafetyDelta { added: vec![payload.safety_requirement.clone()], ..Default::default() }), ..Default::default() }
}
