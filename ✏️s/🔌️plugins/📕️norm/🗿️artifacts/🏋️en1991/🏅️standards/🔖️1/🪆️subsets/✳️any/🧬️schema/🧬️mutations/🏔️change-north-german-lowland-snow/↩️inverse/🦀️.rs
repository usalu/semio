//! Inverse for `change-north-german-lowland-snow`.
use super::ChangeNorthGermanLowlandSnow;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeNorthGermanLowlandSnow, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeNorthGermanLowlandSnow(ChangeNorthGermanLowlandSnow { new_north_german_lowland_snow: base.north_german_lowland_snow })]

    })())
}
