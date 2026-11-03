use crate::remove_retaining_wall::RemoveRetainingWall;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997RetainingWallsRows};

pub fn diff(payload: &RemoveRetainingWall, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.index >= base.retaining_walls.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("retaining-wall #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { retaining_walls: Some(En1997RetainingWallsRows::removal(&base.retaining_walls, payload.index)), ..Default::default() })
}
