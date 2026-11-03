use crate::insert_embedded_wall::InsertEmbeddedWall;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997EmbeddedWallsRows};

pub fn diff(payload: &InsertEmbeddedWall, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.embedded_walls.iter().any(|existing| existing.id == payload.embedded_wall.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("EmbeddedWall id {} already exists.", payload.embedded_wall.id), [payload.embedded_wall.id.clone()]);
    }
    if payload.index > base.embedded_walls.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.embedded_walls.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { embedded_walls: Some(En1997EmbeddedWallsRows::insertion(payload.index, payload.embedded_wall.clone())), ..Default::default() })
}
