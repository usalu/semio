use crate::change_wall_type::ChangeWallType;
use semio_s_artifact_norm_en1996_diff::En1996Diff;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use semio_s_artifact_norm_en1996_diff::{En1996WallDelta, En1996WallPatch};
pub fn diff(payload: &ChangeWallType, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.index >= base.walls.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid wall index."), Vec::<String>::new());
    }
    let wall = &base.walls[payload.index];
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&wall.id, En1996WallPatch { wall_type: Some(payload.new_wall_type), ..Default::default() }), ..Default::default() })
}
