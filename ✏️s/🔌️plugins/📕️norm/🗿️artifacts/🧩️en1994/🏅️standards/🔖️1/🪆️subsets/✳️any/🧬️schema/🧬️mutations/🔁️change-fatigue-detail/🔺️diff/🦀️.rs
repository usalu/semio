//! 🔺️ Diff for `change-fatigue-detail`.
use crate::change_fatigue_detail::ChangeFatigueDetail;
use semio_s_artifact_norm_en1994_diff::En1994Diff;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn diff(payload: &ChangeFatigueDetail, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if base.fatigue_detail == payload.new_fatigue_detail {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff { fatigue_detail: Some(payload.new_fatigue_detail), ..Default::default() })
}
