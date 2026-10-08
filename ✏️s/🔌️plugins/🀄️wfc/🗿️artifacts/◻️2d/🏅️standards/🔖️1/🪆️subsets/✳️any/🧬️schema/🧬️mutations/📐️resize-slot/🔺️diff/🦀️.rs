//! 🔺️ Sparse diff builder for `ResizeSlot` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::{Wfc2dDiff, Wfc2dRowPatch, Wfc2dRows, Wfc2dSlotPatch};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::ResizeSlot, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    let Some(index) = base.slots.iter().position(|slot| slot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.width <= 0.0 || payload.height <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Slot \"{}\" must keep a positive width and height.", payload.id), [payload.id.clone()]);
    }
    let slot = &base.slots[index];
    if slot.width == payload.width && slot.height == payload.height {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Slot \"{}\" already has that size.", payload.id));
    }
    protocol::MutationOutcome::new(Wfc2dDiff { slots: Wfc2dRows { patched: vec![Wfc2dRowPatch { id: slot.id.clone(), patch: Wfc2dSlotPatch { width: Some(payload.width), height: Some(payload.height), ..Default::default() } }], ..Default::default() }, ..Default::default() })
}
