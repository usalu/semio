//! 🔺️ Sparse diff construction for the `create-delivery-constraint` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚚delivery` per Wave C.

use super::CreateDeliveryConstraint;
use crate::diff::ProgramDeliveryDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateDeliveryConstraint, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.delivery_constraint.header.id;
    if base.delivery.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A delivery constraint already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.delivery.len());
    if at > base.delivery.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the delivery constraint list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { delivery: Some(ProgramDeliveryDelta::insertion(at, payload.delivery_constraint.clone())), ..Default::default() })
}
