//! 🔺️ Sparse diff construction for the `replace-cost-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💰costs` per Wave C.

use super::mutation::ReplaceCostRequirement;
use crate::artifacts::program::diff::{ProgramCostsDelta, ProgramCostsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceCostRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.costs.iter().find(|row| row.header.id == payload.cost_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No cost requirement exists with this id.", [payload.cost_requirement.header.id.0.clone()]);
    };
    if existing == &payload.cost_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This cost requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.cost_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { costs: Some(ProgramCostsDelta { patched: vec![ProgramCostsPatchEntry { id: payload.cost_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
