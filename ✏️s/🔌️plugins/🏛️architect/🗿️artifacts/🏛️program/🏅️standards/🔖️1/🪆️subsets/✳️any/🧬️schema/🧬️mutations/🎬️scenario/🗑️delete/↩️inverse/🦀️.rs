//! ↩️ Inverse (undo) construction for the `delete-scenario` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🎬scenarios` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteScenario, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.scenarios.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateScenario(super::super::create_scenario::CreateScenario { scenario: base.scenarios[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
