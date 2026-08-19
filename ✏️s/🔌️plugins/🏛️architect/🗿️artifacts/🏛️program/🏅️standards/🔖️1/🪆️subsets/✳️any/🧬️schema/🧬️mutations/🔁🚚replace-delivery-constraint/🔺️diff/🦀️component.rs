//! 🔺️ Sparse diff construction for the `replace-delivery-constraint` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚚delivery` per Wave C.

use super::mutation::ReplaceDeliveryConstraint;
use crate::artifacts::program::diff::{ProgramDeliveryDelta, ProgramDeliveryPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceDeliveryConstraint, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.delivery.iter().find(|row| row.header.id == payload.delivery_constraint.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No delivery constraint exists with this id.", [payload.delivery_constraint.header.id.0.clone()]);
    };
    if existing == &payload.delivery_constraint {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This delivery constraint already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.delivery_constraint).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { delivery: Some(ProgramDeliveryDelta { patched: vec![ProgramDeliveryPatchEntry { id: payload.delivery_constraint.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
