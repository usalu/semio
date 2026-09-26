//! Inverse for `change-bridge-span`.
use super::ChangeBridgeSpan;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeBridgeSpan, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeBridgeSpan(ChangeBridgeSpan { new_bridge_span: base.bridge_span })]
}
