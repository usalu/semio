//! 🔺️ Sparse diff builder for `CreateSlot` — a real id-keyed upsert into `slots`.

use crate::diff::{Wfc3dDiff, Wfc3dRows};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::CreateSlot, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    if base.slots.iter().any(|slot| slot.id == payload.slot.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A slot with id \"{}\" already exists.", payload.slot.id), [payload.slot.id.clone()]);
    }
    if payload.slot.width <= 0.0 || payload.slot.height <= 0.0 || payload.slot.depth <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Slot \"{}\" must have a positive width, height and depth.", payload.slot.id), [payload.slot.id.clone()]);
    }
    if let Some(pinned) = &payload.slot.pinned_tile_id {
        if !base.tiles.iter().any(|tile| &tile.id == pinned) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \"{}\" pins unknown tile \"{}\".", payload.slot.id, pinned), [pinned.clone()]);
        }
    }
    let canonical = crate::schema::snapshot::canonical_slot_index(base, &payload.slot.id);
    if payload.index != canonical {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Slot \"{}\" must be inserted at its canonical position {canonical}, not {}.", payload.slot.id, payload.index), [payload.slot.id.clone()]);
    }
    protocol::MutationOutcome::new(Wfc3dDiff { slots: Wfc3dRows { added: vec![payload.slot.clone()], ..Default::default() }, ..Default::default() })
}
