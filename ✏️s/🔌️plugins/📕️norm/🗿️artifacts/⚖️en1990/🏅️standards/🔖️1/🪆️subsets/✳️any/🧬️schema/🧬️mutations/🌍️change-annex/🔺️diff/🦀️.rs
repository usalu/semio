//! 🔺️ `change-annex` sparse diff.

use crate::change_annex::ChangeAnnex;
use semio_s_artifact_norm_en1990_diff::En1990Diff;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeAnnex, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.annex == mutation.new_annex {
        return MutationOutcome::empty().warning("mutation.no-op", "annex already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        annex: Some(mutation.new_annex),
        ..En1990Diff::default()
    })
}
