//! Inverse for `change-wind-zone`.
use super::ChangeWindZone;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeWindZone, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeWindZone(ChangeWindZone { new_wind_zone: base.wind_zone })]

    })())
}
