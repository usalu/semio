//! Inverse for `change-fire-duration`.
use super::ChangeFireDuration;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeFireDuration, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeFireDuration(ChangeFireDuration { new_fire_duration: base.fire_duration })]

    })())
}
