//! Inverse for `change-crane-class`.
use super::ChangeCraneClass;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeCraneClass, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeCraneClass(ChangeCraneClass { new_crane_class: base.crane_class.clone() })]
}
