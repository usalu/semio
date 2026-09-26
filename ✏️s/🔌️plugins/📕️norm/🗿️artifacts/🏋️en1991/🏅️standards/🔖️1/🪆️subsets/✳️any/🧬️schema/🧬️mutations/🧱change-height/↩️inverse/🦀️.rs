//! Inverse for `change-height`.
use super::ChangeHeight;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeHeight, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeHeight(ChangeHeight { new_height: base.height })]
}
