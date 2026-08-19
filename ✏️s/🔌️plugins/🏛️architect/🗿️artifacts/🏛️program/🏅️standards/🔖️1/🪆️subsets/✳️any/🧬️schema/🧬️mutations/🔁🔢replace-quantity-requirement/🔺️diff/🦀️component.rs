//! 🔺️ Sparse diff construction for the `replace-quantity-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔢quantities` per Wave C.

use super::mutation::ReplaceQuantityRequirement;
use crate::artifacts::program::diff::{ProgramQuantitiesDelta, ProgramQuantitiesPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceQuantityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.quantities.iter().find(|row| row.header.id == payload.quantity_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No quantity requirement exists with this id.", [payload.quantity_requirement.header.id.0.clone()]);
    };
    if existing == &payload.quantity_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This quantity requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.quantity_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { quantities: Some(ProgramQuantitiesDelta { patched: vec![ProgramQuantitiesPatchEntry { id: payload.quantity_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
