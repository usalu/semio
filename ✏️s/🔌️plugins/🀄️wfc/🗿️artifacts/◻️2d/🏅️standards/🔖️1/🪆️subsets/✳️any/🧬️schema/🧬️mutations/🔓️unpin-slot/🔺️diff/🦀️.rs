//! 🔺️ Sparse diff builder for `UnpinSlot` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::UnpinSlot, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    let Some(index) = base.slots.iter().position(|slot| slot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let slot = &base.slots[index];
    if slot.pinned_tile_id.is_none() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Slot \"{}\" carries no pin.", payload.id));
    }
    let released = crate::schema::snapshot::Wfc2dSlot { pinned_tile_id: None, ..slot.clone() };
    protocol::MutationOutcome::new(Wfc2dDiff { slots_upserted: vec![(index, released)], ..Default::default() })
}
