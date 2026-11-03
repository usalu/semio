//! 🔺️ Diff for `change-beam-sheeting-ribs-parallel`.
use crate::change_beam_sheeting_ribs_parallel::ChangeBeamSheetingRibsParallel;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994BeamsRows, En1994BeamsPatch};

pub fn diff(payload: &ChangeBeamSheetingRibsParallel, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("beam {} does not exist.", payload.id), [payload.id.clone()]);
    };
    let Some(beam) = base.beams.get(index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [index.to_string()]);
    };
    if beam.sheeting.ribs_parallel_to_beam == payload.new_ribs_parallel_to_beam {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        beams: Some(En1994BeamsRows::modification(&payload.id, En1994BeamsPatch { sheeting_ribs_parallel_to_beam: Some(payload.new_ribs_parallel_to_beam), ..Default::default() })),
        ..Default::default()
    })
}
