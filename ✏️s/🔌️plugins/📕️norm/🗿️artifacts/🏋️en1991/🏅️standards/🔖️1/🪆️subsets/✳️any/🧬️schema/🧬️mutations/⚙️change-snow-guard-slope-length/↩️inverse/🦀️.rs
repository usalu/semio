//! ↩️ `change-snow-guard-slope-length` inverse: restores the previous `slope_length`.

use crate::change_snow_guard_slope_length::ChangeSnowGuardSlopeLength;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeSnowGuardSlopeLength, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_snow_guard_slope_length::ChangeSnowGuardSlopeLength> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSnowGuardSlopeLength, base: &En1991Snapshot) -> Option<ChangeSnowGuardSlopeLength> {
    let r0 = base.snow_guards.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSnowGuardSlopeLength { id: payload.id.clone(), new_slope_length: r0.slope_length })
}
