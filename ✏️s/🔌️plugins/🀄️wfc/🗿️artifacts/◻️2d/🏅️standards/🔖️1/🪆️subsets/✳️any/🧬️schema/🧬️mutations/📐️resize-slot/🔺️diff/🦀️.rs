//! 🔺️ Sparse diff builder for `ResizeSlot` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::ResizeSlot, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    let Some(index) = base.slots.iter().position(|slot| slot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.width <= 0.0 || payload.height <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Slot \"{}\" must keep a positive width and height.", payload.id), [payload.id.clone()]);
    }
    let slot = &base.slots[index];
    if slot.width == payload.width && slot.height == payload.height {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Slot \"{}\" already has that size.", payload.id));
    }
    let resized = crate::schema::snapshot::Wfc2dSlot { width: payload.width, height: payload.height, ..slot.clone() };
    protocol::MutationOutcome::new(Wfc2dDiff { slots_upserted: vec![(index, resized)], ..Default::default() })
}
