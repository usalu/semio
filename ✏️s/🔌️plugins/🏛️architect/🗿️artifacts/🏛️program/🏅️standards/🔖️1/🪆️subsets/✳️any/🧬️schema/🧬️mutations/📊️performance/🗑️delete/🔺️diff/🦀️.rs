//! 🔺️ Sparse diff construction for the `delete-performance-criterion` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📊performance` per Wave C.

use super::DeletePerformanceCriterion;
use crate::standards::v1::subsets::any::schema::diff::ProgramPerformanceDelta;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [{id, index}]`.
pub fn diff(payload: &DeletePerformanceCriterion, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(position) = base.performance.iter().position(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No performance criterion exists with this id.", [payload.id.0.clone()]);
    };
    protocol::MutationOutcome::new(ProgramDiff { performance: Some(ProgramPerformanceDelta::removal(&base.performance, position)), ..Default::default() })
}
