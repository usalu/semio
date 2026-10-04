//! Inverse for `change-altitude`.
use super::ChangeAltitude;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAltitude, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeAltitude(ChangeAltitude { new_altitude: base.altitude })]

    })())
}
