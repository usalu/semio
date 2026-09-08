//! ↩️ Inverse (undo) construction for the `delete-performance-criterion` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📊performance` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeletePerformanceCriterion, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.performance.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreatePerformanceCriterion(super::super::create_performance_criterion::CreatePerformanceCriterion { performance_criterion: existing.clone() })],
        None => Vec::new(),
    }
}
