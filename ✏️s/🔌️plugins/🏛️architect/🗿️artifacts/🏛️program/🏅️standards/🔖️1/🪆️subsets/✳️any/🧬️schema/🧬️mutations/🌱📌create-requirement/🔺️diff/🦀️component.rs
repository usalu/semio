//! 🔺️ Sparse diff construction for the `create-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📌requirements` per Wave C.

use super::mutation::CreateRequirement;
use crate::artifacts::program::diff::ProgramRequirementsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.requirements` on apply.
pub fn diff(payload: &CreateRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { requirements: Some(ProgramRequirementsDelta { added: vec![payload.requirement.clone()], ..Default::default() }), ..Default::default() }
}
