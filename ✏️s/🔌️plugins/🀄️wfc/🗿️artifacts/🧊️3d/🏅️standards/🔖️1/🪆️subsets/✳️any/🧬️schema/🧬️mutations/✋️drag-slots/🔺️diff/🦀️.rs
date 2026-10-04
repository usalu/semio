//! 🔺️ Sparse diff builder for `DragSlots` — every addressed slot moved by the offset read off its BASE position,
//! one id-keyed replacement at its own index, in document order.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::{Slot3d, Wfc3dSnapshot};

pub fn diff(payload: &super::DragSlots, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    if !payload.holds_invariants() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag names at least one slot, never one twice, by a finite offset", payload.targets.clone());
    }
    let missing: Vec<String> = payload.targets.iter().filter(|id| !base.slots.iter().any(|slot| &slot.id == *id)).cloned().collect();
    if missing.len() == payload.targets.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is a slot of this document", payload.targets.len()), payload.targets.clone());
    }
    let partial: Vec<protocol::MutationMessage> =
        (!missing.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} target(s) skipped (not in this document): {}", missing.len(), payload.targets.len(), missing.join(", "))).at(missing)).into_iter().collect();
    if payload.is_zero() {
        return protocol::MutationOutcome::new(Wfc3dDiff::default()).absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "a zero offset moves nothing").at(payload.targets.clone())]));
    }
    let slots_upserted = base
        .slots
        .iter()
        .enumerate()
        .filter(|(_, slot)| payload.targets.contains(&slot.id))
        .map(|(index, slot)| (index, Slot3d { x: slot.x + payload.dx, y: slot.y + payload.dy, z: slot.z + payload.dz, ..slot.clone() }))
        .collect();
    protocol::MutationOutcome::new(Wfc3dDiff { slots_upserted, ..Default::default() }).absorb_messages(partial)
}
