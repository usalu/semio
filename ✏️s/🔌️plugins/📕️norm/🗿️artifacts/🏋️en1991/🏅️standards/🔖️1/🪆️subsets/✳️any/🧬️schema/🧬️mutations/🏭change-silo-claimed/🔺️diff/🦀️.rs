//! 🔺️ Diff for `change-silo-claimed`.
use crate::change_silo_claimed::ChangeSiloClaimed;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeSiloClaimed, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.silo_claimed == payload.new_silo_claimed {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_claimed: Some(payload.new_silo_claimed), ..Default::default() })
}
