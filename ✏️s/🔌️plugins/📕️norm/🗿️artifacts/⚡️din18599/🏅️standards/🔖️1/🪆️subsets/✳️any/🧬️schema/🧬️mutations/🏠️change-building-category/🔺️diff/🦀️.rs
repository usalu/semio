//! 🔺️ `change-building-category` sparse diff.

use semio_s_artifact_norm_din18599_diff::Din18599Diff;
use crate::change_building_category::ChangeBuildingCategory;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn diff(payload: &ChangeBuildingCategory, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.building_category == payload.new_building_category {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "building-category already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { building_category: Some(payload.new_building_category), ..Default::default() })
}
