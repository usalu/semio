//! 🔺️ `change-annex` sparse diff.

use super::ChangeAnnex;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeAnnex, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.annex == mutation.new_annex {
        return MutationOutcome::empty().warn("mutation.no-op", "annex already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        annex: Some(mutation.new_annex),
        ..En1990Diff::default()
    })
}
