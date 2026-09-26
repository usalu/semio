//! Inverse for `change-silo-height`.
use super::ChangeSiloHeight;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeSiloHeight, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeSiloHeight(ChangeSiloHeight { new_silo_height: base.silo_height })]
}
