//! ↩️ Inverse (undo) construction for the `create-performance-criterion` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📊performance` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreatePerformanceCriterion, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeletePerformanceCriterion(super::super::delete_performance_criterion::mutation::DeletePerformanceCriterion { id: payload.performance_criterion.header.id.clone() })]
}
