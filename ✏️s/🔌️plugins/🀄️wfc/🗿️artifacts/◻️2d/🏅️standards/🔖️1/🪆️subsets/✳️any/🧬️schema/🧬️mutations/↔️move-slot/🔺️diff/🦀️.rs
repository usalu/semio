//! 🔺️ Sparse diff builder for `MoveSlot` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::{Wfc2dDiff, Wfc2dRowPatch, Wfc2dRows, Wfc2dSlotPatch};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::MoveSlot, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    let Some(index) = base.slots.iter().position(|slot| slot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let slot = &base.slots[index];
    if slot.x == payload.x && slot.y == payload.y {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Slot \"{}\" is already at that position.", payload.id));
    }
    protocol::MutationOutcome::new(Wfc2dDiff { slots: Wfc2dRows { patched: vec![Wfc2dRowPatch { id: slot.id.clone(), patch: Wfc2dSlotPatch { x: Some(payload.x), y: Some(payload.y), ..Default::default() } }], ..Default::default() }, ..Default::default() })
}
