//! 🔺️ Sparse diff builder for `MoveSlot` — one id-keyed replacement at the slot's OWN index.

use crate::diff::{Wfc3dDiff, Wfc3dRowPatch, Wfc3dRows, Wfc3dSlotPatch};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::MoveSlot, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    let Some(index) = base.slots.iter().position(|slot| slot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let slot = &base.slots[index];
    if slot.x == payload.x && slot.y == payload.y && slot.z == payload.z {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Slot \"{}\" is already there.", payload.id));
    }
    protocol::MutationOutcome::new(Wfc3dDiff { slots: Wfc3dRows { patched: vec![Wfc3dRowPatch { id: slot.id.clone(), patch: Wfc3dSlotPatch { x: Some(payload.x), y: Some(payload.y), z: Some(payload.z), ..Default::default() } }], ..Default::default() }, ..Default::default() })
}
