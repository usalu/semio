//! 🏋️ `insert-beam-action` diff on the keyed `actions` lane of `beams`.

use crate::insert_beam_action::InsertBeamAction;
use semio_s_artifact_norm_en1994_diff::{En1994BeamsActionsInserted, En1994BeamsActionsRows, En1994BeamsPatch, En1994BeamsRows, En1994Diff};
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn diff(payload: &InsertBeamAction, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(parent) = base.beams.iter().find(|row| row.id == payload.beam_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Beam {} does not exist.", payload.beam_id), [payload.beam_id.clone()]);
    };
    if parent.actions.iter().any(|existing| existing.id == payload.action.id) {
        let key = payload.action.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Beam action {key} already exists."), [key]);
    }
    if payload.index > parent.actions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, parent.actions.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1994Diff { beams: Some(En1994BeamsRows::modification(&payload.beam_id, En1994BeamsPatch { actions: Some(En1994BeamsActionsRows::insertion(payload.index, payload.action.clone())), ..Default::default() })), ..Default::default() })
}
