//! 🔺️ `change-tension-action-force` diff: patches `actions[].force` of the addressed row's list.

use crate::change_tension_action_force::ChangeTensionActionForce;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993TensionComponentDelta, En1993TensionComponentPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeTensionActionForce, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_force.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.tension_components.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "tension-action", Vec::<String>::new());
    };
    let Some(at) = row.actions.iter().position(|element| element.id == payload.element_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "tension-action", Vec::<String>::new());
    };
    if row.actions[at].force == payload.new_force {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The element already has this value.");
    }
    let mut value = row.actions.clone();
    value[at].force = payload.new_force;
    protocol::MutationOutcome::new(En1993Diff { tension_components: En1993TensionComponentDelta::modification(payload.id.clone(), En1993TensionComponentPatch { actions: Some(value), ..Default::default() }), ..Default::default() })
}
