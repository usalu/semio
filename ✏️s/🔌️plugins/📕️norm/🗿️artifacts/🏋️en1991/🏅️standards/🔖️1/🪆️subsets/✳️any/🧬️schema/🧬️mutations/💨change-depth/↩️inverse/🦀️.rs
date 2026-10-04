//! Inverse for `change-depth`.
use super::ChangeDepth;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeDepth, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeDepth(ChangeDepth { new_depth: base.depth })]

    })())
}
