//! 🔺️ Sparse diff construction for the `rename-quantity-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔢quantities` per Wave C.

use super::mutation::RenameQuantityRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramQuantitiesDelta, ProgramQuantitiesPatchEntry};
use crate::artifacts::program::registers::QuantityRequirementPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameQuantityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = QuantityRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { quantities: Some(ProgramQuantitiesDelta { patched: vec![ProgramQuantitiesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
