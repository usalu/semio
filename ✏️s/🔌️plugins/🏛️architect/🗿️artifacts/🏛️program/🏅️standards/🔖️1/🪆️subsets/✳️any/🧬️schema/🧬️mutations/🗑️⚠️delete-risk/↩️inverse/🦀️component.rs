//! ↩️ Inverse (undo) construction for the `delete-risk` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚠️risks` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::DeleteRisk, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.risks.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateRisk(super::super::create_risk::mutation::CreateRisk { risk: existing.clone() })],
        None => Vec::new(),
    }
}
