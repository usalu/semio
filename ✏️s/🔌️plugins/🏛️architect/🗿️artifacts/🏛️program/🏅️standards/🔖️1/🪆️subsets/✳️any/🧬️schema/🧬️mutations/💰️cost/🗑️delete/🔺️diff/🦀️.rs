//! 🔺️ Sparse diff construction for the `delete-cost-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💰costs` per Wave C.

use super::DeleteCostRequirement;
use crate::standards::v1::subsets::any::schema::diff::ProgramCostsDelta;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [{id, index}]`.
pub fn diff(payload: &DeleteCostRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(position) = base.costs.iter().position(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No cost requirement exists with this id.", [payload.id.0.clone()]);
    };
    protocol::MutationOutcome::new(ProgramDiff { costs: Some(ProgramCostsDelta::removal(&base.costs, position)), ..Default::default() })
}
