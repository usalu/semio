//! 🔺️ Sparse diff construction for the `create-cost-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💰costs` per Wave C.

use super::CreateCostRequirement;
use crate::diff::ProgramCostsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateCostRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.cost_requirement.header.id;
    if base.costs.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A cost requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.costs.len());
    if at > base.costs.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the cost requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { costs: Some(ProgramCostsDelta::insertion(at, payload.cost_requirement.clone())), ..Default::default() })
}
