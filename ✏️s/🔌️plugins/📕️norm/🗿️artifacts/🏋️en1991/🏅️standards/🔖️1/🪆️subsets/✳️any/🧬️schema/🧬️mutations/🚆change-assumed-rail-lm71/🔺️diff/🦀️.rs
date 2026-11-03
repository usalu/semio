//! 🔺️ Diff for `change-assumed-rail-lm71`.
use crate::change_assumed_rail_lm71::ChangeAssumedRailLm71;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAssumedRailLm71, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_rail_lm71 == payload.new_assumed_rail_lm71 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_rail_lm71: Some(payload.new_assumed_rail_lm71), ..Default::default() })
}
