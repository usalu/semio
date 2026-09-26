//! Diff for `change-annex`.
use super::ChangeAnnex;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &ChangeAnnex, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if false {
        return protocol::MutationOutcome::fatal("mutation.invariant", "invalid value", Vec::<String>::new());
    }
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff { annex: Some(payload.new_annex), ..Default::default() })
}
