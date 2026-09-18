//! 🔺️ Sparse diff builder for `PinSlot` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::PinSlot, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    let Some(index) = base.slots.iter().position(|slot| slot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !base.tiles.iter().any(|tile| tile.id == payload.tile_id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tile \"{}\" does not exist.", payload.tile_id), [payload.tile_id.clone()]);
    }
    let slot = &base.slots[index];
    if slot.pinned_tile_id.as_deref() == Some(payload.tile_id.as_str()) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Slot \"{}\" is already pinned to \"{}\".", payload.id, payload.tile_id));
    }
    let pinned = crate::schema::snapshot::Wfc2dSlot { pinned_tile_id: Some(payload.tile_id.clone()), ..slot.clone() };
    protocol::MutationOutcome::new(Wfc2dDiff { slots_upserted: vec![(index, pinned)], ..Default::default() })
}
