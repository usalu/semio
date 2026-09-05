//! 🔺️ Sparse diff construction for the `rename-accessibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♿accessibility` per Wave C.

use super::RenameAccessibilityRequirement;
use crate::artifacts::program::diff::{ProgramAccessibilityDelta, ProgramAccessibilityPatchEntry};
use crate::artifacts::program::registers::AccessibilityRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameAccessibilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.accessibility.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No accessibility requirement exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This accessibility requirement already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = AccessibilityRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { accessibility: Some(ProgramAccessibilityDelta { patched: vec![ProgramAccessibilityPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
