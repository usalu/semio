//! 🔺️ `change-annex` diff.

use crate::diff::En1999Diff;
use crate::mutations::change_annex::ChangeAnnex;
use crate::En1999Snapshot;

pub fn diff(payload: &ChangeAnnex, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "National annex already set.");
    }
    protocol::MutationOutcome::new(En1999Diff { annex: Some(payload.new_annex), ..Default::default() })
}
