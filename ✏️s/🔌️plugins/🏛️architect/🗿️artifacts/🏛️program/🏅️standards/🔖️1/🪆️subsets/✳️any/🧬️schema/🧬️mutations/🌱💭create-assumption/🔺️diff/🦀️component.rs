//! 🔺️ Sparse diff construction for the `create-assumption` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💭assumptions` per Wave C.

use super::mutation::CreateAssumption;
use crate::artifacts::program::diff::ProgramAssumptionsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.assumptions` on apply.
pub fn diff(payload: &CreateAssumption, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { assumptions: Some(ProgramAssumptionsDelta { added: vec![payload.assumption.clone()], ..Default::default() }), ..Default::default() }
}
