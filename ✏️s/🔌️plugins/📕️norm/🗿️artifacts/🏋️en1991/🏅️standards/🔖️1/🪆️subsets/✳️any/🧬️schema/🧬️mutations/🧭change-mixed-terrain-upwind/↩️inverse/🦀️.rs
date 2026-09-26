//! Inverse for `change-mixed-terrain-upwind`.
use super::ChangeMixedTerrainUpwind;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeMixedTerrainUpwind, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeMixedTerrainUpwind(ChangeMixedTerrainUpwind { new_mixed_terrain_upwind: base.mixed_terrain_upwind })]
}
