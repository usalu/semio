//! 🔺️ Sparse diff construction for the `delete-quantity-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔢quantities` per Wave C.

use super::DeleteQuantityRequirement;
use crate::diff::ProgramQuantitiesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DeleteQuantityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.quantities.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No quantity requirement exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { quantities: Some(ProgramQuantitiesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
