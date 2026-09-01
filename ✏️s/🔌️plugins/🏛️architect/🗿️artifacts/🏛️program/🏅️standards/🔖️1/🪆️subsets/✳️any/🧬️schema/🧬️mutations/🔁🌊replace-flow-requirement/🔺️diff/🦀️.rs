//! 🔺️ Sparse diff construction for the `replace-flow-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌊flows` per Wave C.

use super::ReplaceFlowRequirement;
use crate::artifacts::program::diff::{ProgramFlowsDelta, ProgramFlowsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceFlowRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.flows.iter().find(|row| row.header.id == payload.flow_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No flow requirement exists with this id.", [payload.flow_requirement.header.id.0.clone()]);
    };
    if existing == &payload.flow_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This flow requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.flow_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { flows: Some(ProgramFlowsDelta { patched: vec![ProgramFlowsPatchEntry { id: payload.flow_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
