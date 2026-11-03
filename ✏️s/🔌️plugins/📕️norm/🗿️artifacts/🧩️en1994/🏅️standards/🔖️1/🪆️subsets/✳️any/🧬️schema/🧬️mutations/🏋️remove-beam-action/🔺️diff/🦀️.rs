//! 🏋️ `remove-beam-action` diff on the keyed `actions` lane of `beams`.

use crate::remove_beam_action::RemoveBeamAction;
use semio_s_artifact_norm_en1994_diff::{En1994BeamsActionsRemoved, En1994BeamsActionsRows, En1994BeamsPatch, En1994BeamsRows, En1994Diff};
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn diff(payload: &RemoveBeamAction, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(parent) = base.beams.iter().find(|row| row.id == payload.beam_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Beam {} does not exist.", payload.beam_id), [payload.beam_id.clone()]);
    };
    let Some(index) = parent.actions.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Beam action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1994Diff { beams: Some(En1994BeamsRows::modification(&payload.beam_id, En1994BeamsPatch { actions: Some(En1994BeamsActionsRows::removal(&parent.actions, index)), ..Default::default() })), ..Default::default() })
}
