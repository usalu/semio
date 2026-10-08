//! 🔺️ Sparse diff construction for the `create-scenario` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎬scenarios` per Wave C.

use super::CreateScenario;
use crate::diff::ProgramScenariosDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateScenario, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.scenario.header.id;
    if base.scenarios.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A scenario already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.scenarios.len());
    if at > base.scenarios.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the scenario list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { scenarios: Some(ProgramScenariosDelta::insertion(at, payload.scenario.clone())), ..Default::default() })
}
