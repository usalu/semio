//! 🔺️ Diff for `change-annex`.
use crate::change_annex::ChangeAnnex;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAnnex, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { annex: Some(payload.new_annex), ..Default::default() })
}
