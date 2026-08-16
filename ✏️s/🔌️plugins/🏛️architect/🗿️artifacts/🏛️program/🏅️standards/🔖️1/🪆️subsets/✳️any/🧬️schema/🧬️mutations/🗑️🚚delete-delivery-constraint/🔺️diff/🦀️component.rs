//! 🔺️ Sparse diff construction for the `delete-delivery-constraint` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚚delivery` per Wave C.

use super::mutation::DeleteDeliveryConstraint;
use crate::artifacts::program::diff::ProgramDeliveryDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteDeliveryConstraint, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { delivery: Some(ProgramDeliveryDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
