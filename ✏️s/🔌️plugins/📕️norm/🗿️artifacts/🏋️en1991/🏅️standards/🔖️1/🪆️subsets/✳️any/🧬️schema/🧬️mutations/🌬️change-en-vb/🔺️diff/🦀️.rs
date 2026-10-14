//! 🔺️ Diff for `change-en-vb`.
use crate::change_en_vb::ChangeEnVb;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeEnVb, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.en_vb == payload.new_en_vb {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { en_vb: Some(payload.new_en_vb), ..Default::default() })
}
