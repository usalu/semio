//! 🔺️ Sparse diff construction for the `replace-scenario` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎬scenarios` per Wave C.

use super::ReplaceScenario;
use crate::artifacts::program::diff::{ProgramScenariosDelta, ProgramScenariosPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceScenario, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.scenarios.iter().find(|row| row.header.id == payload.scenario.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No scenario exists with this id.", [payload.scenario.header.id.0.clone()]);
    };
    if existing == &payload.scenario {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This scenario already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.scenario).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { scenarios: Some(ProgramScenariosDelta { patched: vec![ProgramScenariosPatchEntry { id: payload.scenario.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
