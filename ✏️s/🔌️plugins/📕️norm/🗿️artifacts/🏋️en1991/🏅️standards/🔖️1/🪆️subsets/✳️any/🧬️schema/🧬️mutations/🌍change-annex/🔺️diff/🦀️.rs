//! Diff for `change-annex`.
use super::ChangeAnnex;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAnnex, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { annex: Some(payload.new_annex), ..Default::default() })
}
