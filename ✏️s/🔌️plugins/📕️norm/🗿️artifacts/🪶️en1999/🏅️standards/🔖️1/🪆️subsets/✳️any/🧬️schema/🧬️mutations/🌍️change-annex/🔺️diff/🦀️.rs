//! 🔺️ `change-annex` diff.

use semio_s_artifact_norm_en1999_diff::En1999Diff;
use crate::change_annex::ChangeAnnex;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeAnnex, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "National annex already set.");
    }
    protocol::MutationOutcome::new(En1999Diff { annex: Some(payload.new_annex), ..Default::default() })
}
