//! 🔺️ Diff for `change-assumed-rail-lm71-udl`.
use crate::change_assumed_rail_lm71_udl::ChangeAssumedRailLm71Udl;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAssumedRailLm71Udl, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_rail_lm71_udl == payload.new_assumed_rail_lm71_udl {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_rail_lm71_udl: Some(payload.new_assumed_rail_lm71_udl), ..Default::default() })
}
