use crate::insert_retaining_wall::InsertRetainingWall;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997RetainingWallsRows};

pub fn diff(payload: &InsertRetainingWall, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.retaining_walls.iter().any(|existing| existing.id == payload.retaining_wall.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("RetainingWall id {} already exists.", payload.retaining_wall.id), [payload.retaining_wall.id.clone()]);
    }
    if payload.index > base.retaining_walls.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.retaining_walls.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { retaining_walls: Some(En1997RetainingWallsRows::insertion(payload.index, payload.retaining_wall.clone())), ..Default::default() })
}
