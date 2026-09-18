//! 🔺️ Sparse diff builder for `UnpinSlot` — one id-keyed replacement at the slot's OWN index.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::UnpinSlot, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    let Some(index) = base.slots.iter().position(|slot| slot.id == payload.id) else {
        return protocol::MutationOutcome::error("wfc3d.slot.missing", format!("Slot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let slot = &base.slots[index];
    if slot.pinned_tile_id.is_none() {
        return protocol::MutationOutcome::empty().warn("wfc3d.slot.pin-absent", format!("Slot \"{}\" carries no pin.", payload.id));
    }
    let mut released = slot.clone();
    released.pinned_tile_id = None;
    protocol::MutationOutcome::new(Wfc3dDiff { slots_upserted: vec![(index, released)], ..Default::default() })
}
