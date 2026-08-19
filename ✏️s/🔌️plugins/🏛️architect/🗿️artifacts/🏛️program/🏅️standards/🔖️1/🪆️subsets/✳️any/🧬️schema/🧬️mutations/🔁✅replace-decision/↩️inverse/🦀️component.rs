//! ↩️ Inverse (undo) construction for the `replace-decision` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `✅decisions` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceDecision, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.decisions.iter().find(|row| row.header.id == payload.decision.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceDecision(super::mutation::ReplaceDecision { decision: existing.clone() })],
        None => Vec::new(),
    }
}
