//! 🔺️ Sparse diff construction for the `replace-infrastructure-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏗️infrastructure` per Wave C.

use super::mutation::ReplaceInfrastructureRequirement;
use crate::artifacts::program::diff::{ProgramInfrastructureDelta, ProgramInfrastructurePatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceInfrastructureRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.infrastructure.iter().find(|row| row.header.id == payload.infrastructure_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No infrastructure requirement exists with this id.", [payload.infrastructure_requirement.header.id.0.clone()]);
    };
    if existing == &payload.infrastructure_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This infrastructure requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.infrastructure_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { infrastructure: Some(ProgramInfrastructureDelta { patched: vec![ProgramInfrastructurePatchEntry { id: payload.infrastructure_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
