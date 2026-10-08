//! 🔺️ Sparse diff builder for `DragSlots` — every addressed slot moved by the offset read off its BASE position,
//! one field patch of its coordinates.

use crate::diff::{Wfc3dDiff, Wfc3dRowPatch, Wfc3dRows, Wfc3dSlotPatch};
use crate::schema::snapshot::Wfc3dSnapshot;

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
    let patched: Vec<Wfc3dRowPatch<Wfc3dSlotPatch>> = base
        .slots
        .iter()
        .filter(|slot| payload.targets.contains(&slot.id))
        .map(|slot| Wfc3dRowPatch { id: slot.id.clone(), patch: Wfc3dSlotPatch { x: Some(slot.x + payload.dx), y: Some(slot.y + payload.dy), z: Some(slot.z + payload.dz), ..Default::default() } })
        .collect();
    protocol::MutationOutcome::new(Wfc3dDiff { slots: Wfc3dRows { patched, ..Default::default() }, ..Default::default() }).absorb_messages(partial)
}
