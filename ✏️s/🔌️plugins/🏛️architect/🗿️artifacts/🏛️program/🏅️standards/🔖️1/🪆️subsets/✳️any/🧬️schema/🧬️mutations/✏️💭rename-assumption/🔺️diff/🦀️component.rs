//! 🔺️ Sparse diff construction for the `rename-assumption` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💭assumptions` per Wave C.

use super::mutation::RenameAssumption;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramAssumptionsDelta, ProgramAssumptionsPatchEntry};
use crate::artifacts::program::registers::AssumptionPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameAssumption, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = AssumptionPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { assumptions: Some(ProgramAssumptionsDelta { patched: vec![ProgramAssumptionsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
