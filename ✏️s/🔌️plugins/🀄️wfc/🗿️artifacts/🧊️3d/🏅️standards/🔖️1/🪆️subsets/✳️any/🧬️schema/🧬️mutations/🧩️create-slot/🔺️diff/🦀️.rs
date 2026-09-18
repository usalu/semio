//! 🔺️ Sparse diff builder for `CreateSlot` — a real id-keyed upsert into `slots`.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::CreateSlot, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    if base.slots.iter().any(|slot| slot.id == payload.slot.id) {
        return protocol::MutationOutcome::fatal("wfc3d.slot.duplicate-id", format!("A slot with id \"{}\" already exists.", payload.slot.id), [payload.slot.id.clone()]);
    }
    if payload.slot.width <= 0.0 || payload.slot.height <= 0.0 || payload.slot.depth <= 0.0 {
        return protocol::MutationOutcome::fatal("wfc3d.slot.degenerate-box", format!("Slot \"{}\" must have a positive width, height and depth.", payload.slot.id), [payload.slot.id.clone()]);
    }
    if let Some(pinned) = &payload.slot.pinned_tile_id {
        if !base.tiles.iter().any(|tile| &tile.id == pinned) {
            return protocol::MutationOutcome::fatal("wfc3d.slot.unknown-pinned-tile", format!("Slot \"{}\" pins unknown tile \"{}\".", payload.slot.id, pinned), [pinned.clone()]);
        }
    }
    protocol::MutationOutcome::new(Wfc3dDiff { slots_upserted: vec![(payload.index, payload.slot.clone())], ..Default::default() })
}
