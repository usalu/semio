//! 🔺️ Sparse diff construction for the `delete-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📌requirements` per Wave C.

use super::mutation::DeleteRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramRequirementsDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { requirements: Some(ProgramRequirementsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
