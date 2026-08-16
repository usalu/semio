//! 🔺️ Sparse diff construction for the `create-operational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📋operations` per Wave C.

use super::mutation::CreateOperationalRequirement;
use crate::artifacts::program::diff::ProgramOperationsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.operations` on apply.
pub fn diff(payload: &CreateOperationalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { operations: Some(ProgramOperationsDelta { added: vec![payload.operational_requirement.clone()], ..Default::default() }), ..Default::default() }
}
