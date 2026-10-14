use crate::change_eccentricity_bottom::ChangeEccentricityBottom;
use semio_s_artifact_norm_en1996_diff::En1996Diff;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use semio_s_artifact_norm_en1996_diff::{En1996WallDelta, En1996WallPatch};
pub fn diff(payload: &ChangeEccentricityBottom, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.index >= base.walls.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid wall index."), Vec::<String>::new());
    }
    let wall = &base.walls[payload.index];
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&wall.id, En1996WallPatch { eccentricity_bottom_m: Some(payload.new_eccentricity_bottom_m), ..Default::default() }), ..Default::default() })
}
