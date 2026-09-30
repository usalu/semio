//! Diff for `change-terrain-category`.
use super::ChangeTerrainCategory;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeTerrainCategory, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.terrain_category == payload.new_terrain_category {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { terrain_category: Some(payload.new_terrain_category), ..Default::default() })
}
