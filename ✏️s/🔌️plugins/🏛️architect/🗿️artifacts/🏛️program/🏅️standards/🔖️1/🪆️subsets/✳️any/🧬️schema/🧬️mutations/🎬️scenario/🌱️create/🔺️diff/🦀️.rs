//! 🔺️ Sparse diff construction for the `create-scenario` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎬scenarios` per Wave C.

use super::CreateScenario;
use crate::diff::ProgramScenariosDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateScenario, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.scenario.header.id;
    if base.scenarios.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A scenario already exists with this id.", [id.0.clone()]);
    }
    let length = base.scenarios.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the scenario list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.scenarios.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { scenarios: Some(ProgramScenariosDelta { added: vec![payload.scenario.clone()], reordered, ..Default::default() }), ..Default::default() })
}
