//! Diff for `change-mixed-terrain-distance`.
use super::ChangeMixedTerrainDistance;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeMixedTerrainDistance, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.mixed_terrain_distance == payload.new_mixed_terrain_distance {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { mixed_terrain_distance: Some(payload.new_mixed_terrain_distance), ..Default::default() })
}
