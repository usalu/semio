//! 🔺️ Sparse diff construction for the `replace-quantity-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔢quantities` per Wave C.

use super::ReplaceQuantityRequirement;
use crate::diff::ProgramQuantitiesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceQuantityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.quantity_requirement.header.id;
    let Some(position) = base.quantities.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No quantity requirement exists with this id.", [id.0.clone()]);
    };
    if base.quantities[position] == payload.quantity_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This quantity requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramQuantitiesDelta::removal(&base.quantities, position);
    delta.absorb(ProgramQuantitiesDelta::insertion(position, payload.quantity_requirement.clone()));
    protocol::MutationOutcome::new(ProgramDiff { quantities: Some(delta), ..Default::default() })
}
