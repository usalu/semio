//! 🔺️ Diff for `change-beam-crack-width-limit-m`.
use crate::change_beam_crack_width_limit_m::ChangeBeamCrackWidthLimitM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994BeamsRows, En1994BeamsPatch};

pub fn diff(payload: &ChangeBeamCrackWidthLimitM, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("beam {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.new_wk_limit_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [index.to_string()]);
    }
    let Some(beam) = base.beams.get(index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [index.to_string()]);
    };
    if (beam.wk_limit_m - payload.new_wk_limit_m).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        beams: Some(En1994BeamsRows::modification(&payload.id, En1994BeamsPatch { wk_limit_m: Some(payload.new_wk_limit_m), ..Default::default() })),
        ..Default::default()
    })
}
