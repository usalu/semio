//! ✏️ `change-tank-content-category` diff: patches `content_category` of the addressed tank row.

use crate::change_tank_content_category::ChangeTankContentCategory;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TankDelta, En1998TankPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeTankContentCategory, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.tanks.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tank {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.content_category == payload.new_content_category {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tank already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { tanks: En1998TankDelta::modification(&payload.id, En1998TankPatch { content_category: Some(payload.new_content_category), ..Default::default() }), ..Default::default() })
}
