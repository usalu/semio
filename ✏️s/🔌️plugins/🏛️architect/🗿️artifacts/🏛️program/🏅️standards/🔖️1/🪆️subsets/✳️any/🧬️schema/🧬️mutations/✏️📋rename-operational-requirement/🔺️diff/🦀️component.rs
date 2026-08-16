//! 🔺️ Sparse diff construction for the `rename-operational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📋operations` per Wave C.

use super::mutation::RenameOperationalRequirement;
use crate::artifacts::program::diff::{ProgramOperationsDelta, ProgramOperationsPatchEntry};
use crate::artifacts::program::registers::OperationalRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameOperationalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = OperationalRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { operations: Some(ProgramOperationsDelta { patched: vec![ProgramOperationsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
