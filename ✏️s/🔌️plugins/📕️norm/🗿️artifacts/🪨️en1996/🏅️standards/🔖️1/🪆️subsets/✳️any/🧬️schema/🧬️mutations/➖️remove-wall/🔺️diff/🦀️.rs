use crate::remove_wall::RemoveWall;
use semio_s_artifact_norm_en1996_diff::En1996Diff;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use semio_s_artifact_norm_en1996_diff::En1996WallDelta;
pub fn diff(payload: &RemoveWall, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.index >= base.walls.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid remove index."), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::removal(&base.walls, payload.index), ..Default::default() })
}
