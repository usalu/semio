//! 🔺️ Diff for `change-fire-load-basis`.
use crate::change_fire_load_basis::ChangeFireLoadBasis;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeFireLoadBasis, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fire_load_basis == payload.new_fire_load_basis {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_load_basis: Some(payload.new_fire_load_basis), ..Default::default() })
}
