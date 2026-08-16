//! 🔺️ Sparse diff construction for the `create-delivery-constraint` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚚delivery` per Wave C.

use super::mutation::CreateDeliveryConstraint;
use crate::artifacts::program::diff::ProgramDeliveryDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateDeliveryConstraint, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.delivery_constraint.header.id.clone();
    if base.delivery.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A delivery constraint already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { delivery: Some(ProgramDeliveryDelta { added: vec![payload.delivery_constraint.clone()], ..Default::default() }), ..Default::default() })
}
