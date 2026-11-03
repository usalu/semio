//! ➖️ `remove-tension-action` diff: removes the element at the index from the addressed row's list.

use crate::remove_tension_action::RemoveTensionAction;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993TensionComponentDelta, En1993TensionComponentPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveTensionAction, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.tension_components.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "tension-action", Vec::<String>::new());
    };
    if payload.index >= row.actions.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("tension-action index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut value = row.actions.clone();
    value.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { tension_components: En1993TensionComponentDelta::modification(payload.id.clone(), En1993TensionComponentPatch { actions: Some(value), ..Default::default() }), ..Default::default() })
}
