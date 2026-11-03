//! 🔺️ Diff for `change-rail-alpha`.
use crate::change_rail_alpha::ChangeRailAlpha;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeRailAlpha, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.rail_alpha == payload.new_rail_alpha {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { rail_alpha: Some(payload.new_rail_alpha), ..Default::default() })
}
