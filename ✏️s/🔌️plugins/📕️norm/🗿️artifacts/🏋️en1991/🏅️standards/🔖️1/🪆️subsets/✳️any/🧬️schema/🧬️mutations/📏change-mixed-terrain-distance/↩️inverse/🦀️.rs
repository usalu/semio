//! Inverse for `change-mixed-terrain-distance`.
use super::ChangeMixedTerrainDistance;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeMixedTerrainDistance, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeMixedTerrainDistance(ChangeMixedTerrainDistance { new_mixed_terrain_distance: base.mixed_terrain_distance })]
}
