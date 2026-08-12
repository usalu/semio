//! ↩️ Inverse (undo) construction for the `delete-performance-criterion` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📊performance` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeletePerformanceCriterion, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.performance.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreatePerformanceCriterion(super::super::create_performance_criterion::mutation::CreatePerformanceCriterion { performance_criterion: existing.clone() })],
        None => Vec::new(),
    }
}
