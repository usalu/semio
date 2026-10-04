//! Inverse for `change-assumed-bridge-lm4`.
use super::ChangeAssumedBridgeLm4;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAssumedBridgeLm4, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeAssumedBridgeLm4(ChangeAssumedBridgeLm4 { new_assumed_bridge_lm4: base.assumed_bridge_lm4 })]

    })())
}
