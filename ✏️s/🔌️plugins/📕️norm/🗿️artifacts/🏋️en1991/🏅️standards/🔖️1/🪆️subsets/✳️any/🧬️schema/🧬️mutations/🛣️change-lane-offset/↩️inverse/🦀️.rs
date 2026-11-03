//! ↩️ `change-lane-offset` inverse: restores the previous `offset`.

use crate::change_lane_offset::ChangeLaneOffset;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeLaneOffset, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lane_offset::ChangeLaneOffset> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeLaneOffset, base: &En1991Snapshot) -> Option<ChangeLaneOffset> {
    let r0 = base.bridge_lanes.iter().find(|row| row.id == payload.id)?;
    Some(ChangeLaneOffset { id: payload.id.clone(), new_offset: r0.offset })
}
