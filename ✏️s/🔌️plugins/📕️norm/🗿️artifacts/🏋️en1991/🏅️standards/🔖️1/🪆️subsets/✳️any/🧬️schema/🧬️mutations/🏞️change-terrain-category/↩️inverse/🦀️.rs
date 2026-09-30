//! Inverse for `change-terrain-category`.
use super::ChangeTerrainCategory;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeTerrainCategory, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeTerrainCategory(ChangeTerrainCategory { new_terrain_category: base.terrain_category })]
}
