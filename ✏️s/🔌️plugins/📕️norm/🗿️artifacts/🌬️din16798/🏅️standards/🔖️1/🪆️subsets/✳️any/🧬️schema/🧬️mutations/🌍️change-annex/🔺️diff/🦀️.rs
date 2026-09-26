//! 🔺️ `change-annex` diff.
use super::ChangeAnnex;
use crate::{Din16798Diff, Din16798Snapshot};
pub fn diff(payload: &ChangeAnnex, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(Din16798Diff { annex: Some(payload.new_annex.clone()), ..Default::default() })
}
