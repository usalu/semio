//! 🔺️ Sparse diff construction for the `delete-delivery-constraint` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚚delivery` per Wave C.

use super::mutation::DeleteDeliveryConstraint;
use crate::artifacts::program::diff::ProgramDeliveryDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteDeliveryConstraint, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.delivery.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No delivery constraint exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { delivery: Some(ProgramDeliveryDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
