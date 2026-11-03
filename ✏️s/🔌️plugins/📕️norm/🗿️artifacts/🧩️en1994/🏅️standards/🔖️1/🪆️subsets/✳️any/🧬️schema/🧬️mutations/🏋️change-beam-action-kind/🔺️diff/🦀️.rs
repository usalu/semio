//! 🔺️ Diff for `change-beam-action-kind`.
use crate::change_beam_action_kind::ChangeBeamActionKind;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994BeamsRows, En1994BeamsPatch, En1994BeamsActionsRows, En1994BeamsActionsPatch};

pub fn diff(payload: &ChangeBeamActionKind, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.beam_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("beam {} does not exist.", payload.beam_id), [payload.beam_id.clone()]);
    };
    let Some(action_index) = base.beams[index].actions.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    let Some(beam) = base.beams.get(index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [index.to_string()]);
    };
    let Some(action) = beam.actions.get(action_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "action missing", [action_index.to_string()]);
    };
    if action.kind == payload.new_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        beams: Some(En1994BeamsRows::modification(&payload.beam_id, En1994BeamsPatch {
                actions: Some(En1994BeamsActionsRows::modification(&payload.id, En1994BeamsActionsPatch { kind: Some(payload.new_kind), ..Default::default() })),
                ..Default::default()
            })),
        ..Default::default()
    })
}
