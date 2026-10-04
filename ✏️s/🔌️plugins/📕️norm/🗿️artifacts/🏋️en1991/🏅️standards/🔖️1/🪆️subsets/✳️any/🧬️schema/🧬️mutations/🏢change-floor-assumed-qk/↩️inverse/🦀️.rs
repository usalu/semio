//! Inverse for `change-floor-assumed-qk`.
use super::ChangeFloorAssumedQk;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &ChangeFloorAssumedQk, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.floors.len() { return Vec::new(); }
    vec![En1991Mutation::ChangeFloorAssumedQk(ChangeFloorAssumedQk { index: payload.index, new_assumed_qk: base.floors[payload.index].assumed_qk })]

    })())
}
