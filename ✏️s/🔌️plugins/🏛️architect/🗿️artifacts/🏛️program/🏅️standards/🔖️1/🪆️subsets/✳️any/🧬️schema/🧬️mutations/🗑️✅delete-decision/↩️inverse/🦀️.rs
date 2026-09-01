//! ↩️ Inverse (undo) construction for the `delete-decision` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `✅decisions` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeleteDecision, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.decisions.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateDecision(super::super::create_decision::CreateDecision { decision: existing.clone() })],
        None => Vec::new(),
    }
}
