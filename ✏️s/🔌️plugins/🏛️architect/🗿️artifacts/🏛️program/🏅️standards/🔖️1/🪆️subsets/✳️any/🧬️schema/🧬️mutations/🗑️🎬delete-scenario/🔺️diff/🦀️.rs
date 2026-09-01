//! 🔺️ Sparse diff construction for the `delete-scenario` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎬scenarios` per Wave C.

use super::DeleteScenario;
use crate::artifacts::program::diff::ProgramScenariosDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteScenario, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.scenarios.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No scenario exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { scenarios: Some(ProgramScenariosDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
