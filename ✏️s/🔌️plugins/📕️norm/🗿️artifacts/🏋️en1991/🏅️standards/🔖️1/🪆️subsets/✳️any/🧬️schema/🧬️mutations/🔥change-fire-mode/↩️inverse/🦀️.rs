//! Inverse for `change-fire-mode`.
use super::ChangeFireMode;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeFireMode, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeFireMode(ChangeFireMode { new_fire_mode: base.fire_mode })]

    })())
}
