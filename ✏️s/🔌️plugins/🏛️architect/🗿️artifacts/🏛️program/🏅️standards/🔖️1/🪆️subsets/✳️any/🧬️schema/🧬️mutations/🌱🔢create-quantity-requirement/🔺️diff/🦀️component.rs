//! 🔺️ Sparse diff construction for the `create-quantity-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔢quantities` per Wave C.

use super::mutation::CreateQuantityRequirement;
use crate::artifacts::program::diff::ProgramQuantitiesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.quantities` on apply.
pub fn diff(payload: &CreateQuantityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { quantities: Some(ProgramQuantitiesDelta { added: vec![payload.quantity_requirement.clone()], ..Default::default() }), ..Default::default() }
}
