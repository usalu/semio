//! 🔺️ Sparse diff construction for the `create-sustainability-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♻️sustainability` per Wave C.

use super::mutation::CreateSustainabilityRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSustainabilityDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.sustainability` on apply.
pub fn diff(payload: &CreateSustainabilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { sustainability: Some(ProgramSustainabilityDelta { added: vec![payload.sustainability_requirement.clone()], ..Default::default() }), ..Default::default() }
}
