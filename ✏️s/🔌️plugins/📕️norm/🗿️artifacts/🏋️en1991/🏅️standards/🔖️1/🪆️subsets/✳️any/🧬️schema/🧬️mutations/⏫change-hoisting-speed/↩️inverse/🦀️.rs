//! Inverse for `change-hoisting-speed`.
use super::ChangeHoistingSpeed;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeHoistingSpeed, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeHoistingSpeed(ChangeHoistingSpeed { new_hoisting_speed: base.hoisting_speed })]

    })())
}
