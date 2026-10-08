//! 📎️ `change-annex` diff — sets the national annex; an annex the document already holds is a `mutation.no-op`.
use super::ChangeAnnex;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &ChangeAnnex, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "annex unchanged");
    }
    protocol::MutationOutcome::new(En1998Diff { annex: Some(payload.new_annex.clone()), ..Default::default() })
}
