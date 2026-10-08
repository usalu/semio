//! 🔺️ Sparse diff construction for the `create-delivery-constraint` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚚delivery` per Wave C.

use super::CreateDeliveryConstraint;
use crate::diff::ProgramDeliveryDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateDeliveryConstraint, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.delivery_constraint.header.id;
    if base.delivery.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A delivery constraint already exists with this id.", [id.0.clone()]);
    }
    let length = base.delivery.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the delivery constraint list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.delivery.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { delivery: Some(ProgramDeliveryDelta { added: vec![payload.delivery_constraint.clone()], reordered, ..Default::default() }), ..Default::default() })
}
