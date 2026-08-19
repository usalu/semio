//! 🔺️ Sparse diff construction for the `rename-scenario` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎬scenarios` per Wave C.

use super::mutation::RenameScenario;
use crate::artifacts::program::diff::{ProgramScenariosDelta, ProgramScenariosPatchEntry};
use crate::artifacts::program::registers::ScenarioPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameScenario, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.scenarios.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No scenario exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This scenario already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = ScenarioPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { scenarios: Some(ProgramScenariosDelta { patched: vec![ProgramScenariosPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
