//! 🔺️ Diff for `change-en-sk`.
use crate::change_en_sk::ChangeEnSk;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeEnSk, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.en_sk == payload.new_en_sk {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { en_sk: Some(payload.new_en_sk), ..Default::default() })
}
