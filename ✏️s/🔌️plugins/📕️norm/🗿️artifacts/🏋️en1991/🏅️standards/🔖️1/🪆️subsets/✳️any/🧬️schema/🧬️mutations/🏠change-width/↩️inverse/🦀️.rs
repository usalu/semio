//! Inverse for `change-width`.
use super::ChangeWidth;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeWidth, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeWidth(ChangeWidth { new_width: base.width })]
}
