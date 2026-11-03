//! 🔺️ Diff for `change-beam-steel-width-m`.
use crate::change_beam_steel_width_m::ChangeBeamSteelWidthM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994BeamsRows, En1994BeamsPatch};

pub fn diff(payload: &ChangeBeamSteelWidthM, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("beam {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.new_width_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [index.to_string()]);
    }
    let Some(beam) = base.beams.get(index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [index.to_string()]);
    };
    if (beam.steel.width_m - payload.new_width_m).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        beams: Some(En1994BeamsRows::modification(&payload.id, En1994BeamsPatch { steel_width_m: Some(payload.new_width_m), ..Default::default() })),
        ..Default::default()
    })
}
