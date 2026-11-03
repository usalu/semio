//! 📏 `change-concentrated-bearing-length` diff: patches `bearing_length_m` of the addressed concentrated row.

use crate::change_concentrated_bearing_length::ChangeConcentratedBearingLength;
use semio_s_artifact_norm_en1996_diff::{En1996ConcentratedDelta, En1996ConcentratedPatch, En1996Diff, En1996LoadCaseDelta, En1996LoadCasePatch, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeConcentratedBearingLength, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_bearing_length_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.wall_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.wall_id), [payload.wall_id.clone()]);
    };
    let Some(r1) = r0.load_cases.iter().find(|row| row.id == payload.load_case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("load case {} does not exist.", payload.load_case_id), [payload.load_case_id.clone()]);
    };
    let Some(r2) = r1.concentrated.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("concentrated {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r2.bearing_length_m == payload.new_bearing_length_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The concentrated already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.wall_id, En1996WallPatch { load_cases: En1996LoadCaseDelta::modification(&payload.load_case_id, En1996LoadCasePatch { concentrated: En1996ConcentratedDelta::modification(&payload.id, En1996ConcentratedPatch { bearing_length_m: Some(payload.new_bearing_length_m), ..Default::default() }), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
