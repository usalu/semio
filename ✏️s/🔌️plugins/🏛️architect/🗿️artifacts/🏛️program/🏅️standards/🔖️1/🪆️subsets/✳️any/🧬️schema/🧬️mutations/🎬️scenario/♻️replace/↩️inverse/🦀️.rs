//! ↩️ Inverse (undo) construction for the `replace-scenario` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🎬scenarios` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceScenario, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.scenarios.iter().find(|row| row.header.id == payload.scenario.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceScenario(super::ReplaceScenario { scenario: existing.clone() })],
        None => Vec::new(),
    }
}
