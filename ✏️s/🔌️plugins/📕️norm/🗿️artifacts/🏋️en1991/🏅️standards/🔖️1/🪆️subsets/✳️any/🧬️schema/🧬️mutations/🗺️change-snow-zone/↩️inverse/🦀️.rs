//! Inverse for `change-snow-zone`.
use super::ChangeSnowZone;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeSnowZone, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeSnowZone(ChangeSnowZone { new_snow_zone: base.snow_zone.clone() })]

    })())
}
