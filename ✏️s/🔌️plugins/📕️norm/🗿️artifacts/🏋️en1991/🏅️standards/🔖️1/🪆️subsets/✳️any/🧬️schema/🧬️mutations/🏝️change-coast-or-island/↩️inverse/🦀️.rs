//! Inverse for `change-coast-or-island`.
use super::ChangeCoastOrIsland;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeCoastOrIsland, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeCoastOrIsland(ChangeCoastOrIsland { new_coast_or_island: base.coast_or_island })]
}
