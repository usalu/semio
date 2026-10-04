//! 🔺️ `change-annex` diff — sets the document's annex; the same value is a `mutation.no-op`.

use super::ChangeAnnex;
use crate::{En1996Diff, En1996Snapshot};

pub fn diff(payload: &ChangeAnnex, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The annex already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { annex: Some(payload.new_annex), ..Default::default() })
}
