//! ↩️ `change-climate-zone` inverse.

use crate::change_climate_zone::ChangeClimateZone;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(_payload: &ChangeClimateZone, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_climate_zone::ChangeClimateZone> {
    Ok((|| {
    vec![Op::from(ChangeClimateZone { new_climate_zone: base.climate_zone })]

    })())
}
