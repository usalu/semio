//! 🔺️ Diff for `change-beam-steel-root-radius-m`.
use crate::change_beam_steel_root_radius_m::ChangeBeamSteelRootRadiusM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994BeamsRows, En1994BeamsPatch};

pub fn diff(payload: &ChangeBeamSteelRootRadiusM, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("beam {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.new_r_m.is_finite() || payload.new_r_m < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite and not negative", [index.to_string()]);
    }
    let Some(beam) = base.beams.get(index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [index.to_string()]);
    };
    if (beam.steel.r_m - payload.new_r_m).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        beams: Some(En1994BeamsRows::modification(&payload.id, En1994BeamsPatch { steel_r_m: Some(payload.new_r_m), ..Default::default() })),
        ..Default::default()
    })
}
