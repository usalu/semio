use crate::remove_embedded_wall::RemoveEmbeddedWall;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997EmbeddedWallsRows};

pub fn diff(payload: &RemoveEmbeddedWall, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.index >= base.embedded_walls.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("embedded-wall #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { embedded_walls: Some(En1997EmbeddedWallsRows::removal(&base.embedded_walls, payload.index)), ..Default::default() })
}
