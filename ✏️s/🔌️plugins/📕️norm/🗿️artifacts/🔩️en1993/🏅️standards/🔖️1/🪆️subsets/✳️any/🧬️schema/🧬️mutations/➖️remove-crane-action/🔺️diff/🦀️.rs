//! ➖️ `remove-crane-action` diff: removes the element at the index from the addressed row's list.

use crate::remove_crane_action::RemoveCraneAction;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993CraneRunwayDelta, En1993CraneRunwayPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveCraneAction, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.crane_runways.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "crane-action", Vec::<String>::new());
    };
    if payload.index >= row.actions.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("crane-action index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut value = row.actions.clone();
    value.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { crane_runways: En1993CraneRunwayDelta::modification(payload.id.clone(), En1993CraneRunwayPatch { actions: Some(value), ..Default::default() }), ..Default::default() })
}
