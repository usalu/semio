//! 🔺️ Sparse diff construction for the `delete-operational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📋operations` per Wave C.

use super::mutation::DeleteOperationalRequirement;
use crate::artifacts::program::diff::ProgramOperationsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteOperationalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { operations: Some(ProgramOperationsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
