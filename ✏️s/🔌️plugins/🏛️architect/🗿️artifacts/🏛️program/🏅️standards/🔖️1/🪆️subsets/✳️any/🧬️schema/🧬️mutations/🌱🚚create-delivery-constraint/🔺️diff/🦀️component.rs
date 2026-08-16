//! 🔺️ Sparse diff construction for the `create-delivery-constraint` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚚delivery` per Wave C.

use super::mutation::CreateDeliveryConstraint;
use crate::artifacts::program::diff::ProgramDeliveryDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.delivery` on apply.
pub fn diff(payload: &CreateDeliveryConstraint, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { delivery: Some(ProgramDeliveryDelta { added: vec![payload.delivery_constraint.clone()], ..Default::default() }), ..Default::default() }
}
