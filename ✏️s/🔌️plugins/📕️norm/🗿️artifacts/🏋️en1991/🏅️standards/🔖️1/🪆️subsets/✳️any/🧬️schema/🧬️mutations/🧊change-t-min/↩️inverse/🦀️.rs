//! Inverse for `change-t-min`.
use super::ChangeTMin;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeTMin, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeTMin(ChangeTMin { new_t_min: base.t_min })]
}
