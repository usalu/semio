//! 🏷️ `change-wall-label-en` diff: patches `label_en` of the addressed wall row.

use crate::change_wall_label_en::ChangeWallLabelEn;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeWallLabelEn, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.label_en == payload.new_label_en {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The wall already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.id, En1996WallPatch { label_en: Some(payload.new_label_en.clone()), ..Default::default() }), ..Default::default() })
}
