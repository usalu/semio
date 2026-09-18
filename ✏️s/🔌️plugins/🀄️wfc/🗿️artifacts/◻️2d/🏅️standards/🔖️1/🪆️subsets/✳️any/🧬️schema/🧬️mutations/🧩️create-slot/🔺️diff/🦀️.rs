//! 🔺️ Sparse diff builder for `CreateSlot` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::CreateSlot, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if base.slots.iter().any(|slot| slot.id == payload.slot.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A slot with id \"{}\" already exists.", payload.slot.id), [payload.slot.id.clone()]);
    }
    if payload.slot.width <= 0.0 || payload.slot.height <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Slot \"{}\" must have a positive width and height.", payload.slot.id), [payload.slot.id.clone()]);
    }
    if let Some(pinned) = &payload.slot.pinned_tile_id {
        if !base.tiles.iter().any(|tile| &tile.id == pinned) {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Slot \"{}\" pins unknown tile \"{}\".", payload.slot.id, pinned), [pinned.clone()]);
        }
    }
    let at = crate::mutations::ordered_index(&base.slots, &payload.slot.id, |slot| slot.id.as_str());
    protocol::MutationOutcome::new(Wfc2dDiff { slots_upserted: vec![(at, payload.slot.clone())], ..Default::default() })
}
