//! 🔺️ Diff for `change-annex`.
use crate::change_annex::ChangeAnnex;
use semio_s_artifact_norm_en1994_diff::En1994Diff;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn diff(payload: &ChangeAnnex, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if false {
        return protocol::MutationOutcome::fatal("mutation.invariant", "invalid value", Vec::<String>::new());
    }
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff { annex: Some(payload.new_annex), ..Default::default() })
}
