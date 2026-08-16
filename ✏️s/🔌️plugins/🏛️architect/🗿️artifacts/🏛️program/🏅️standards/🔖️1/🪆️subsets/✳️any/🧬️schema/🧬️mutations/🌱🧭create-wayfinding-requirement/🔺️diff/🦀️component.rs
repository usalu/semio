//! 🔺️ Sparse diff construction for the `create-wayfinding-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧭wayfinding` per Wave C.

use super::mutation::CreateWayfindingRequirement;
use crate::artifacts::program::diff::ProgramWayfindingDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateWayfindingRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.wayfinding_requirement.header.id.clone();
    if base.wayfinding.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A wayfinding requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { wayfinding: Some(ProgramWayfindingDelta { added: vec![payload.wayfinding_requirement.clone()], ..Default::default() }), ..Default::default() })
}
