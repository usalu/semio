//! ✏️ `change-silo-content-category` diff: patches `content_category` of the addressed silo row.

use crate::change_silo_content_category::ChangeSiloContentCategory;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998SiloDelta, En1998SiloPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeSiloContentCategory, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.silos.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("silo {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.content_category == payload.new_content_category {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The silo already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { silos: En1998SiloDelta::modification(&payload.id, En1998SiloPatch { content_category: Some(payload.new_content_category), ..Default::default() }), ..Default::default() })
}
