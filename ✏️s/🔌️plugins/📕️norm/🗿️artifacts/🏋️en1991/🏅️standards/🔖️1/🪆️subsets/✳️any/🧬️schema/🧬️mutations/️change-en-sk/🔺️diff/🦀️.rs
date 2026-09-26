//! Diff for `change-en-sk`.
use super::ChangeEnSk;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeEnSk, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.en_sk == payload.new_en_sk {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { en_sk: Some(payload.new_en_sk), ..Default::default() })
}
