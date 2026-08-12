//! 🔺️ Sparse diff construction for the `delete-assumption` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💭assumptions` per Wave C.

use super::mutation::DeleteAssumption;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramAssumptionsDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteAssumption, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { assumptions: Some(ProgramAssumptionsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
