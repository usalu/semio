//! 🔺️ Sparse diff construction for the `rename-delivery-constraint` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚚delivery` per Wave C.

use super::mutation::RenameDeliveryConstraint;
use crate::artifacts::program::diff::{ProgramDeliveryDelta, ProgramDeliveryPatchEntry};
use crate::artifacts::program::registers::DeliveryConstraintPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameDeliveryConstraint, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = DeliveryConstraintPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { delivery: Some(ProgramDeliveryDelta { patched: vec![ProgramDeliveryPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
