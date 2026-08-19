//! ↩️ Inverse (undo) construction for the `create-scenario` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🎬scenarios` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateScenario, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteScenario(super::super::delete_scenario::mutation::DeleteScenario { id: payload.scenario.header.id.clone() })]
}
