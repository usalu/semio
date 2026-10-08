//! 🔺️ Sparse diff builder for `SetSlotPositions` — one field patch per slot whose position changes.

use crate::diff::{Wfc2dDiff, Wfc2dSlotPatch, Wfc2dSlotsDelta, Wfc2dSlotsModification};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::SetSlotPositions, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if !payload.holds_invariants() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "positions name at least one slot, never one twice, at finite coordinates", payload.ids());
    }
    let missing: Vec<String> = payload.positions.iter().filter(|position| !base.slots.iter().any(|slot| slot.id == position.id)).map(|position| position.id.clone()).collect();
    if missing.len() == payload.positions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} positioned slot(s) is a slot of this document", payload.positions.len()), payload.ids());
    }
    let partial: Vec<protocol::MutationMessage> =
        (!missing.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} position(s) skipped (not in this document): {}", missing.len(), payload.positions.len(), missing.join(", "))).at(missing)).into_iter().collect();
    let patched: Vec<Wfc2dSlotsModification> = base
        .slots
        .iter()
        .filter_map(|slot| {
            let position = payload.positions.iter().find(|position| position.id == slot.id)?;
            ((position.x, position.y) != (slot.x, slot.y)).then(|| Wfc2dSlotsModification { id: slot.id.clone(), patch: Wfc2dSlotPatch { x: Some(position.x), y: Some(position.y), ..Default::default() } })
        })
        .collect();
    if patched.is_empty() {
        return protocol::MutationOutcome::new(Wfc2dDiff::default()).absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "every positioned slot already sits there").at(payload.ids())]));
    }
    protocol::MutationOutcome::new(Wfc2dDiff { slots: Wfc2dSlotsDelta { modified: patched, ..Default::default() }, ..Default::default() }).absorb_messages(partial)
}
