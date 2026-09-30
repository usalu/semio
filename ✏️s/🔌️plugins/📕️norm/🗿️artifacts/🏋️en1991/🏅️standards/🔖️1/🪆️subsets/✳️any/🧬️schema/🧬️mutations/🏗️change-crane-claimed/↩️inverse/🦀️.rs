//! Inverse for `change-crane-claimed`.
use super::ChangeCraneClaimed;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeCraneClaimed, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeCraneClaimed(ChangeCraneClaimed { new_crane_claimed: base.crane_claimed })]
}
