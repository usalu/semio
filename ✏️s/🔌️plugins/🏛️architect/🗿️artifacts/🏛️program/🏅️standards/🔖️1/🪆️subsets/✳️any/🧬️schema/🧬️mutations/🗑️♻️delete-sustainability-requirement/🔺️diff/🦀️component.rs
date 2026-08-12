//! 🔺️ Sparse diff construction for the `delete-sustainability-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♻️sustainability` per Wave C.

use super::mutation::DeleteSustainabilityRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSustainabilityDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteSustainabilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { sustainability: Some(ProgramSustainabilityDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
