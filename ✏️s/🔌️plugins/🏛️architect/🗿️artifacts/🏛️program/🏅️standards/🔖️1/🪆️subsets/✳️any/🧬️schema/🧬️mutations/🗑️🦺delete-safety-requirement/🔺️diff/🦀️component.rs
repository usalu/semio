//! 🔺️ Sparse diff construction for the `delete-safety-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🦺safety` per Wave C.

use super::mutation::DeleteSafetyRequirement;
use crate::artifacts::program::diff::ProgramSafetyDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteSafetyRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { safety: Some(ProgramSafetyDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
