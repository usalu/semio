//! 🔺️ Sparse diff builder for `CreateSlot` — a real id-keyed upsert into `slots` (never a
//! whole-snapshot capture).

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn diff(payload: &super::CreateSlot, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
    if base.slots.iter().any(|slot| slot.id == payload.slot.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A slot with id \"{}\" already exists.", payload.slot.id), [payload.slot.id.clone()]);
    }
    if let Some(pinned) = &payload.slot.pinned_module_id {
        if !base.modules.iter().any(|module| &module.child_id == pinned) {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Slot \"{}\" pins unknown module \"{}\".", payload.slot.id, pinned), [pinned.clone()]);
        }
    }
    protocol::MutationOutcome::new(AssemblyDiff { slots_upserted: vec![(payload.index, payload.slot.clone())], ..Default::default() })
}
