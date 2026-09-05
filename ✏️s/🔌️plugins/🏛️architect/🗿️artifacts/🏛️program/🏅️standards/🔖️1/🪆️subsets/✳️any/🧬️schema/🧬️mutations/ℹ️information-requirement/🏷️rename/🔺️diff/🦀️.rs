//! 🔺️ Sparse diff construction for the `rename-information-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `ℹ️information` per Wave C.

use super::RenameInformationRequirement;
use crate::artifacts::program::diff::{ProgramInformationDelta, ProgramInformationPatchEntry};
use crate::artifacts::program::registers::InformationRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameInformationRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.information.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No information requirement exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This information requirement already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = InformationRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { information: Some(ProgramInformationDelta { patched: vec![ProgramInformationPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
