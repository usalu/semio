//! ↩️ `change-climate-zone` inverse.

use super::ChangeClimateZone;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeClimateZone, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Din4108Mutation::ChangeClimateZone(ChangeClimateZone { new_climate_zone: base.climate_zone })]

    })())
}
