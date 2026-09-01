//! 🔺️ Sparse diff construction for the `create-quantity-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔢quantities` per Wave C.

use super::CreateQuantityRequirement;
use crate::artifacts::program::diff::ProgramQuantitiesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateQuantityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.quantity_requirement.header.id.clone();
    if base.quantities.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A quantity requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { quantities: Some(ProgramQuantitiesDelta { added: vec![payload.quantity_requirement.clone()], ..Default::default() }), ..Default::default() })
}
