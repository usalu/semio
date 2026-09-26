//! Inverse for `change-silo-claimed`.
use super::ChangeSiloClaimed;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeSiloClaimed, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeSiloClaimed(ChangeSiloClaimed { new_silo_claimed: base.silo_claimed })]
}
