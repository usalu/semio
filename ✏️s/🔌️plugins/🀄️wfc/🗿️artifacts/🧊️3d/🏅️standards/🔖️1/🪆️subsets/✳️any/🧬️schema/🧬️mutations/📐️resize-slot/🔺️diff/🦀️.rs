//! 🔺️ Sparse diff builder for `ResizeSlot` — one id-keyed replacement at the slot's OWN index.
//! Guard order: target-missing → invariant → no-op → apply.

use crate::diff::{Wfc3dDiff, Wfc3dRowPatch, Wfc3dRows, Wfc3dSlotPatch};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::ResizeSlot, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    let Some(index) = base.slots.iter().position(|slot| slot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.width <= 0.0 || payload.height <= 0.0 || payload.depth <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Slot \"{}\" must have a positive width, height and depth.", payload.id), [payload.id.clone()]);
    }
    let slot = &base.slots[index];
    if slot.width == payload.width && slot.height == payload.height && slot.depth == payload.depth {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Slot \"{}\" already has that extent.", payload.id));
    }
    protocol::MutationOutcome::new(Wfc3dDiff { slots: Wfc3dRows { patched: vec![Wfc3dRowPatch { id: slot.id.clone(), patch: Wfc3dSlotPatch { width: Some(payload.width), height: Some(payload.height), depth: Some(payload.depth), ..Default::default() } }], ..Default::default() }, ..Default::default() })
}
