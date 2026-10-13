//! 🔺️ Sparse diff construction for the `delete-scenario` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎬scenarios` per Wave C.

use super::DeleteScenario;
use crate::standards::v1::subsets::any::schema::diff::ProgramScenariosDelta;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [{id, index}]`.
pub fn diff(payload: &DeleteScenario, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(position) = base.scenarios.iter().position(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No scenario exists with this id.", [payload.id.0.clone()]);
    };
    protocol::MutationOutcome::new(ProgramDiff { scenarios: Some(ProgramScenariosDelta::removal(&base.scenarios, position)), ..Default::default() })
}
