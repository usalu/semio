//! 🔺️ Sparse diff construction for the `rename-flexibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧩flexibility` per Wave C.

use super::mutation::RenameFlexibilityRequirement;
use crate::artifacts::program::diff::{ProgramFlexibilityDelta, ProgramFlexibilityPatchEntry};
use crate::artifacts::program::registers::FlexibilityRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameFlexibilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.flexibility.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No flexibility requirement exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This flexibility requirement already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = FlexibilityRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { flexibility: Some(ProgramFlexibilityDelta { patched: vec![ProgramFlexibilityPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
