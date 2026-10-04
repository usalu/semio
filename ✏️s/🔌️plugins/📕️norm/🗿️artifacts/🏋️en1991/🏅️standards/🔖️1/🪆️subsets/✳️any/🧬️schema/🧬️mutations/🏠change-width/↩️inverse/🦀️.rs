//! Inverse for `change-width`.
use super::ChangeWidth;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeWidth, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeWidth(ChangeWidth { new_width: base.width })]

    })())
}
