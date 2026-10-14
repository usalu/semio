//! 🔺️ `change-cellar-area` diff.
use crate::change_cellar_area::ChangeCellarArea;
use semio_s_artifact_norm_din16798_diff::Din16798Diff;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn diff(payload: &ChangeCellarArea, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.cellar_area_m2 == payload.new_cellar_area_m2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(Din16798Diff { cellar_area_m2: Some(payload.new_cellar_area_m2), ..Default::default() })
}
