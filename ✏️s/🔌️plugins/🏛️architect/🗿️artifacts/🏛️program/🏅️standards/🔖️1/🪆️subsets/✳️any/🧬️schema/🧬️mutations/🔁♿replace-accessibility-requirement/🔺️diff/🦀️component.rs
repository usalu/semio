//! 🔺️ Sparse diff construction for the `replace-accessibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♿accessibility` per Wave C.

use super::mutation::ReplaceAccessibilityRequirement;
use crate::artifacts::program::diff::{ProgramAccessibilityDelta, ProgramAccessibilityPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceAccessibilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.accessibility.iter().find(|row| row.header.id == payload.accessibility_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No accessibility requirement exists with this id.", [payload.accessibility_requirement.header.id.0.clone()]);
    };
    if existing == &payload.accessibility_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This accessibility requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.accessibility_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { accessibility: Some(ProgramAccessibilityDelta { patched: vec![ProgramAccessibilityPatchEntry { id: payload.accessibility_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
