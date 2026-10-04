//! Inverse for `change-crane-claimed`.
use super::ChangeCraneClaimed;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeCraneClaimed, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeCraneClaimed(ChangeCraneClaimed { new_crane_claimed: base.crane_claimed })]

    })())
}
