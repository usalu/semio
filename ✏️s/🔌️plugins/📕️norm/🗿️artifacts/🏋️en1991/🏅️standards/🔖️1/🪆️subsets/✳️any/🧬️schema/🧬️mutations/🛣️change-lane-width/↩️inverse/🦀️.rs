//! ↩️ `change-lane-width` inverse: restores the previous `width`.

use crate::change_lane_width::ChangeLaneWidth;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeLaneWidth, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lane_width::ChangeLaneWidth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeLaneWidth, base: &En1991Snapshot) -> Option<ChangeLaneWidth> {
    let r0 = base.bridge_lanes.iter().find(|row| row.id == payload.id)?;
    Some(ChangeLaneWidth { id: payload.id.clone(), new_width: r0.width })
}
