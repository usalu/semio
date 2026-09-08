//! 🔺️ Sparse diff construction for the `create-scenario` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎬scenarios` per Wave C.

use super::CreateScenario;
use crate::diff::ProgramScenariosDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateScenario, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.scenario.header.id.clone();
    if base.scenarios.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A scenario already exists with this id.", [id.0]);
    }
    protocol::MutationOutcome::new(ProgramDiff { scenarios: Some(ProgramScenariosDelta { added: vec![payload.scenario.clone()], ..Default::default() }), ..Default::default() })
}
