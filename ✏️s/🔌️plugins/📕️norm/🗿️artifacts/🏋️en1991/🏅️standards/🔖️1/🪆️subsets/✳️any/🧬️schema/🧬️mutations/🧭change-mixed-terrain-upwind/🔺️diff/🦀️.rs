//! Diff for `change-mixed-terrain-upwind`.
use super::ChangeMixedTerrainUpwind;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeMixedTerrainUpwind, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.mixed_terrain_upwind == payload.new_mixed_terrain_upwind {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { mixed_terrain_upwind: Some(payload.new_mixed_terrain_upwind), ..Default::default() })
}
