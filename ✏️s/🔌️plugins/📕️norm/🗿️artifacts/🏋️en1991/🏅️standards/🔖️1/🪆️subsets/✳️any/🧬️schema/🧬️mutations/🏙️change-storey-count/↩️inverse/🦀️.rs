//! Inverse for `change-storey-count`.
use super::ChangeStoreyCount;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeStoreyCount, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeStoreyCount(ChangeStoreyCount { new_storey_count: base.storey_count })]

    })())
}
