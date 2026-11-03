//! 🔺️ `change-tower-action-load-case-id` diff: patches `actions[].load_case_id` of the addressed row's list.

use crate::change_tower_action_load_case_id::ChangeTowerActionLoadCaseId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993TowerLegDelta, En1993TowerLegPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeTowerActionLoadCaseId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.tower_legs.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "tower-action", Vec::<String>::new());
    };
    let Some(at) = row.actions.iter().position(|element| element.id == payload.element_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "tower-action", Vec::<String>::new());
    };
    if row.actions[at].load_case_id == payload.new_load_case_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The element already has this value.");
    }
    let mut value = row.actions.clone();
    value[at].load_case_id = payload.new_load_case_id.clone();
    protocol::MutationOutcome::new(En1993Diff { tower_legs: En1993TowerLegDelta::modification(payload.id.clone(), En1993TowerLegPatch { actions: Some(value), ..Default::default() }), ..Default::default() })
}
