//! Diff for `change-annex`.
use super::ChangeAnnex;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &ChangeAnnex, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut next = base.clone();
    next.annex = payload.new_annex.clone();
    if next.annex == base.annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "annex unchanged");
    }
    protocol::MutationOutcome::new(En1998Diff { annex: Some(payload.new_annex.clone()), ..Default::default() })
}
