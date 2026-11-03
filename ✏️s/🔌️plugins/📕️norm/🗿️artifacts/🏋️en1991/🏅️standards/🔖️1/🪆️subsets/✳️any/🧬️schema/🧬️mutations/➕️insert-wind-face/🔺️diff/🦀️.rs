//! ➕️ `insert-wind-face` diff: inserts the wind face row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_wind_face::InsertWindFace;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991WindFaceDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertWindFace, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.wind_faces.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("wind face {key} already exists."), [key]);
    }
    if payload.index > base.wind_faces.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.wind_faces.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { wind_faces: En1991WindFaceDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
