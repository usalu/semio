//! 🔺️ Sparse diff construction for the `rename-wayfinding-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧭wayfinding` per Wave C.

use super::RenameWayfindingRequirement;
use crate::artifacts::program::diff::{ProgramWayfindingDelta, ProgramWayfindingPatchEntry};
use crate::artifacts::program::registers::WayfindingRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameWayfindingRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.wayfinding.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No wayfinding requirement exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This wayfinding requirement already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = WayfindingRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { wayfinding: Some(ProgramWayfindingDelta { patched: vec![ProgramWayfindingPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
