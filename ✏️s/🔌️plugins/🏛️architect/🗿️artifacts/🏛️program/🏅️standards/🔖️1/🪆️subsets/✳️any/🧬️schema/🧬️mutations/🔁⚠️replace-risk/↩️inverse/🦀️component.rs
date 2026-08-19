//! ↩️ Inverse (undo) construction for the `replace-risk` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚠️risks` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceRisk, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.risks.iter().find(|row| row.header.id == payload.risk.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceRisk(super::mutation::ReplaceRisk { risk: existing.clone() })],
        None => Vec::new(),
    }
}
