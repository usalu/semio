//! 🔺️ Diff for `change-terrain-category`.
use crate::change_terrain_category::ChangeTerrainCategory;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeTerrainCategory, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.terrain_category == payload.new_terrain_category {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { terrain_category: Some(payload.new_terrain_category), ..Default::default() })
}
