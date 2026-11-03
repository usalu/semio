//! 🔺️ Diff for `change-beam-bridge-lanes`.
use crate::change_beam_bridge_lanes::ChangeBeamBridgeLanes;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994BeamsRows, En1994BeamsPatch};

pub fn diff(payload: &ChangeBeamBridgeLanes, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("beam {} does not exist.", payload.id), [payload.id.clone()]);
    };
    let Some(beam) = base.beams.get(index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [index.to_string()]);
    };
    if beam.bridge_lanes == payload.new_bridge_lanes {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        beams: Some(En1994BeamsRows::modification(&payload.id, En1994BeamsPatch { bridge_lanes: Some(payload.new_bridge_lanes), ..Default::default() })),
        ..Default::default()
    })
}
