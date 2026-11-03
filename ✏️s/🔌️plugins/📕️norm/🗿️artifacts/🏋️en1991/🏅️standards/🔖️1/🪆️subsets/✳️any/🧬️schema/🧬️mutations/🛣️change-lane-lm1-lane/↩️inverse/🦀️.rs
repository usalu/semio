//! ↩️ `change-lane-lm1-lane` inverse: restores the previous `lm1_lane`.

use crate::change_lane_lm1_lane::ChangeLaneLm1Lane;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeLaneLm1Lane, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lane_lm1_lane::ChangeLaneLm1Lane> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeLaneLm1Lane, base: &En1991Snapshot) -> Option<ChangeLaneLm1Lane> {
    let r0 = base.bridge_lanes.iter().find(|row| row.id == payload.id)?;
    Some(ChangeLaneLm1Lane { id: payload.id.clone(), new_lm1_lane: r0.lm1_lane })
}
