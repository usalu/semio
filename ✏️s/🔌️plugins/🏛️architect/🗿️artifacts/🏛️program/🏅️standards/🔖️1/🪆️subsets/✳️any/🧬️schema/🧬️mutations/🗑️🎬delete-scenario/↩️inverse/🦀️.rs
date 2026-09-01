//! ↩️ Inverse (undo) construction for the `delete-scenario` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🎬scenarios` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeleteScenario, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.scenarios.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateScenario(super::super::create_scenario::CreateScenario { scenario: existing.clone() })],
        None => Vec::new(),
    }
}
