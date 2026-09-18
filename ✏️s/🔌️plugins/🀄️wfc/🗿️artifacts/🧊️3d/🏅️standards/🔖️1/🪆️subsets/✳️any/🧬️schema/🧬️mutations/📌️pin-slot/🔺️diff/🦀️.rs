//! 🔺️ Sparse diff builder for `PinSlot` — one id-keyed replacement at the slot's OWN index.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::PinSlot, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    let Some(index) = base.slots.iter().position(|slot| slot.id == payload.id) else {
        return protocol::MutationOutcome::error("wfc3d.slot.missing", format!("Slot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !base.tiles.iter().any(|tile| tile.id == payload.tile_id) {
        return protocol::MutationOutcome::error("wfc3d.tile.missing", format!("Tile \"{}\" does not exist.", payload.tile_id), [payload.tile_id.clone()]);
    }
    let slot = &base.slots[index];
    if slot.pinned_tile_id.as_deref() == Some(payload.tile_id.as_str()) {
        return protocol::MutationOutcome::empty().warn("wfc3d.slot.pin-unchanged", format!("Slot \"{}\" is already pinned to \"{}\".", payload.id, payload.tile_id));
    }
    let mut pinned = slot.clone();
    pinned.pinned_tile_id = Some(payload.tile_id.clone());
    protocol::MutationOutcome::new(Wfc3dDiff { slots_upserted: vec![(index, pinned)], ..Default::default() })
}
