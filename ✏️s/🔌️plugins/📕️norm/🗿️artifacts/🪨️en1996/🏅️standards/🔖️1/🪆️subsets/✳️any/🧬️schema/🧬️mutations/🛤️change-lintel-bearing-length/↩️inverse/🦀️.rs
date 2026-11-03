//! ↩️ `change-lintel-bearing-length` inverse: restores the previous `bearing_length_m`.

use crate::change_lintel_bearing_length::ChangeLintelBearingLength;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeLintelBearingLength, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lintel_bearing_length::ChangeLintelBearingLength> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeLintelBearingLength, base: &En1996Snapshot) -> Option<ChangeLintelBearingLength> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.lintels.iter().find(|row| row.id == payload.id)?;
    Some(ChangeLintelBearingLength { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_bearing_length_m: r1.bearing_length_m })
}
