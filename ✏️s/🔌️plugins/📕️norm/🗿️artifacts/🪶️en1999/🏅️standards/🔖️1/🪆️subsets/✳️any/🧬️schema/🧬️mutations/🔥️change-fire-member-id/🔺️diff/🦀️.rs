//! 🔺️ `change-fire-member-id` diff: patches `member_id` of the addressed fire scenario row.

use crate::change_fire_member_id::ChangeFireMemberId;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FireScenariosPatch, En1999FireScenariosRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeFireMemberId, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.fire_scenarios.iter().find(|row| row.id == payload.fire_scenario_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fire-scenario", Vec::<String>::new());
    };
    if r0.member_id == payload.new_member_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fire scenario already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { fire_scenarios: Some(En1999FireScenariosRows::modification(&payload.fire_scenario_id, En1999FireScenariosPatch { member_id: Some(payload.new_member_id.clone()), ..Default::default() })), ..Default::default() })
}
