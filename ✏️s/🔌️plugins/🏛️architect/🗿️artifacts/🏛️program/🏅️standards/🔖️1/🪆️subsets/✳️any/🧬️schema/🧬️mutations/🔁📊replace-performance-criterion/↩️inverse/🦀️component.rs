//! ↩️ Inverse (undo) construction for the `replace-performance-criterion` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📊performance` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplacePerformanceCriterion, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.performance.iter().find(|row| row.header.id == payload.performance_criterion.header.id) {
        Some(existing) => vec![ProgramMutation::ReplacePerformanceCriterion(super::mutation::ReplacePerformanceCriterion { performance_criterion: existing.clone() })],
        None => Vec::new(),
    }
}
