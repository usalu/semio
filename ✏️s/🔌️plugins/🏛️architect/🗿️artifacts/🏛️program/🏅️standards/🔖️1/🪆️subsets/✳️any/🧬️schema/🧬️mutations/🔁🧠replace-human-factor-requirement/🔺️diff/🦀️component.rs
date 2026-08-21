//! 🔺️ Sparse diff construction for the `replace-human-factor-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧠human-factors` per Wave C.

use super::mutation::ReplaceHumanFactorRequirement;
use crate::artifacts::program::diff::{ProgramHumanFactorsDelta, ProgramHumanFactorsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceHumanFactorRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.human_factors.iter().find(|row| row.header.id == payload.human_factor_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No human factor requirement exists with this id.", [payload.human_factor_requirement.header.id.0.clone()]);
    };
    if existing == &payload.human_factor_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This human factor requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.human_factor_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff {
        human_factors: Some(ProgramHumanFactorsDelta { patched: vec![ProgramHumanFactorsPatchEntry { id: payload.human_factor_requirement.header.id.0.clone(), patch }], ..Default::default() }),
        ..Default::default()
    })
}
