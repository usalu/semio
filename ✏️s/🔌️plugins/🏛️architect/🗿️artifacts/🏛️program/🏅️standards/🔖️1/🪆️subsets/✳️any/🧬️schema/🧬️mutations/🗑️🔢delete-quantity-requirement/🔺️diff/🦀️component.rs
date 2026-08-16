//! 🔺️ Sparse diff construction for the `delete-quantity-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔢quantities` per Wave C.

use super::mutation::DeleteQuantityRequirement;
use crate::artifacts::program::diff::ProgramQuantitiesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteQuantityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { quantities: Some(ProgramQuantitiesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
