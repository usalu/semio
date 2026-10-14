//! 🔺️ `change-annex` diff.
use crate::change_annex::ChangeAnnex;
use semio_s_artifact_norm_din16798_diff::Din16798Diff;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn diff(payload: &ChangeAnnex, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(Din16798Diff { annex: Some(payload.new_annex.clone()), ..Default::default() })
}
