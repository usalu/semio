//! 🔺️ Sparse diff builder for `ConnectSlots` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::{Wfc2dDiff, Wfc2dRows};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::ConnectSlots, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if base.edges.iter().any(|edge| edge.id == payload.edge.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An edge with id \"{}\" already exists.", payload.edge.id), [payload.edge.id.clone()]);
    }
    for endpoint in [&payload.edge.from_slot_id, &payload.edge.to_slot_id] {
        if !base.slots.iter().any(|slot| &slot.id == endpoint) {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Edge \"{}\" references unknown slot \"{}\".", payload.edge.id, endpoint), [endpoint.clone()]);
        }
    }
    if payload.edge.relation.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Edge \"{}\" must name a relation.", payload.edge.id), [payload.edge.id.clone()]);
    }
    protocol::MutationOutcome::new(Wfc2dDiff { edges: Wfc2dRows { added: vec![payload.edge.clone()], ..Default::default() }, ..Default::default() })
}
