//! 🔺️ Sparse diff builder for `MoveSlot` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::MoveSlot, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    let Some(index) = base.slots.iter().position(|slot| slot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let slot = &base.slots[index];
    if slot.x == payload.x && slot.y == payload.y {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Slot \"{}\" is already at that position.", payload.id));
    }
    let moved = crate::schema::snapshot::Wfc2dSlot { x: payload.x, y: payload.y, ..slot.clone() };
    protocol::MutationOutcome::new(Wfc2dDiff { slots_upserted: vec![(index, moved)], ..Default::default() })
}
