//! 🔺️ Sparse diff builder for `MoveSlot` — one id-keyed replacement at the slot's OWN index.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::MoveSlot, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    let Some(index) = base.slots.iter().position(|slot| slot.id == payload.id) else {
        return protocol::MutationOutcome::error("wfc3d.slot.missing", format!("Slot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let slot = &base.slots[index];
    if slot.x == payload.x && slot.y == payload.y && slot.z == payload.z {
        return protocol::MutationOutcome::empty().warn("wfc3d.slot.position-unchanged", format!("Slot \"{}\" is already there.", payload.id));
    }
    let mut moved = slot.clone();
    moved.x = payload.x;
    moved.y = payload.y;
    moved.z = payload.z;
    protocol::MutationOutcome::new(Wfc3dDiff { slots_upserted: vec![(index, moved)], ..Default::default() })
}
