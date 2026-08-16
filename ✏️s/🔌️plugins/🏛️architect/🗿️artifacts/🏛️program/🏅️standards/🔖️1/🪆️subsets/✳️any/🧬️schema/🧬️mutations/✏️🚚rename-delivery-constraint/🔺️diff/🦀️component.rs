//! 🔺️ Sparse diff construction for the `rename-delivery-constraint` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚚delivery` per Wave C.

use super::mutation::RenameDeliveryConstraint;
use crate::artifacts::program::diff::{ProgramDeliveryDelta, ProgramDeliveryPatchEntry};
use crate::artifacts::program::registers::DeliveryConstraintPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameDeliveryConstraint, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.delivery.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No delivery constraint exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This delivery constraint already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = DeliveryConstraintPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { delivery: Some(ProgramDeliveryDelta { patched: vec![ProgramDeliveryPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
