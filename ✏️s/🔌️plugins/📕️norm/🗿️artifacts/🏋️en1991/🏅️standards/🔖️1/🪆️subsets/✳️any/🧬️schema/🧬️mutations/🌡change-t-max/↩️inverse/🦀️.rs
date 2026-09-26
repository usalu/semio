//! Inverse for `change-t-max`.
use super::ChangeTMax;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeTMax, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeTMax(ChangeTMax { new_t_max: base.t_max })]
}
