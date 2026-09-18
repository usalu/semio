//! 🔺️ Sparse diff builder for `ResizeSlot` — one id-keyed replacement at the slot's OWN index.
//! Guard order: target-missing → invariant → no-op → apply.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::ResizeSlot, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    let Some(index) = base.slots.iter().position(|slot| slot.id == payload.id) else {
        return protocol::MutationOutcome::error("wfc3d.slot.missing", format!("Slot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.width <= 0.0 || payload.height <= 0.0 || payload.depth <= 0.0 {
        return protocol::MutationOutcome::fatal("wfc3d.slot.degenerate-box", format!("Slot \"{}\" must have a positive width, height and depth.", payload.id), [payload.id.clone()]);
    }
    let slot = &base.slots[index];
    if slot.width == payload.width && slot.height == payload.height && slot.depth == payload.depth {
        return protocol::MutationOutcome::empty().warn("wfc3d.slot.extent-unchanged", format!("Slot \"{}\" already has that extent.", payload.id));
    }
    let mut resized = slot.clone();
    resized.width = payload.width;
    resized.height = payload.height;
    resized.depth = payload.depth;
    protocol::MutationOutcome::new(Wfc3dDiff { slots_upserted: vec![(index, resized)], ..Default::default() })
}
