//! 🔺️ Sparse diff construction for the `replace-delivery-constraint` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚚delivery` per Wave C.

use super::ReplaceDeliveryConstraint;
use crate::diff::ProgramDeliveryDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceDeliveryConstraint, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.delivery_constraint.header.id;
    let Some(position) = base.delivery.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No delivery constraint exists with this id.", [id.0.clone()]);
    };
    if base.delivery[position] == payload.delivery_constraint {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This delivery constraint already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.delivery.len()).then(|| base.delivery.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { delivery: Some(ProgramDeliveryDelta { removed: vec![id.0.clone()], added: vec![payload.delivery_constraint.clone()], reordered, ..Default::default() }), ..Default::default() })
}
