//! Inverse for `change-assumed-bridge-footway`.
use super::ChangeAssumedBridgeFootway;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAssumedBridgeFootway, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeAssumedBridgeFootway(ChangeAssumedBridgeFootway { new_assumed_bridge_footway: base.assumed_bridge_footway })]

    })())
}
