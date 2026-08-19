//! 🔺️ Sparse diff construction for the `replace-resilience-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💪resilience` per Wave C.

use super::mutation::ReplaceResilienceRequirement;
use crate::artifacts::program::diff::{ProgramResilienceDelta, ProgramResiliencePatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceResilienceRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.resilience.iter().find(|row| row.header.id == payload.resilience_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No resilience requirement exists with this id.", [payload.resilience_requirement.header.id.0.clone()]);
    };
    if existing == &payload.resilience_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This resilience requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.resilience_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { resilience: Some(ProgramResilienceDelta { patched: vec![ProgramResiliencePatchEntry { id: payload.resilience_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
