//! 🔺️ Sparse diff construction for the `rename-resilience-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💪resilience` per Wave C.

use super::mutation::RenameResilienceRequirement;
use crate::artifacts::program::diff::{ProgramResilienceDelta, ProgramResiliencePatchEntry};
use crate::artifacts::program::registers::ResilienceRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameResilienceRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.resilience.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No resilience requirement exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This resilience requirement already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = ResilienceRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { resilience: Some(ProgramResilienceDelta { patched: vec![ProgramResiliencePatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
