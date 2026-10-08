//! 🔺️ Sparse diff construction for the `replace-scenario` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎬scenarios` per Wave C.

use super::ReplaceScenario;
use crate::diff::ProgramScenariosDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceScenario, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.scenario.header.id;
    let Some(position) = base.scenarios.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No scenario exists with this id.", [id.0.clone()]);
    };
    if base.scenarios[position] == payload.scenario {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This scenario already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramScenariosDelta::removal(&base.scenarios, position);
    delta.absorb(ProgramScenariosDelta::insertion(position, payload.scenario.clone()));
    protocol::MutationOutcome::new(ProgramDiff { scenarios: Some(delta), ..Default::default() })
}
