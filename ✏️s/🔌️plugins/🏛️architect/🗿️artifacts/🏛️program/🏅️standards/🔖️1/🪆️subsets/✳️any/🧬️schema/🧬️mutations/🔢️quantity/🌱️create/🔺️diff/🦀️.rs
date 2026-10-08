//! 🔺️ Sparse diff construction for the `create-quantity-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔢quantities` per Wave C.

use super::CreateQuantityRequirement;
use crate::diff::ProgramQuantitiesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateQuantityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.quantity_requirement.header.id;
    if base.quantities.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A quantity requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.quantities.len());
    if at > base.quantities.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the quantity requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { quantities: Some(ProgramQuantitiesDelta::insertion(at, payload.quantity_requirement.clone())), ..Default::default() })
}
