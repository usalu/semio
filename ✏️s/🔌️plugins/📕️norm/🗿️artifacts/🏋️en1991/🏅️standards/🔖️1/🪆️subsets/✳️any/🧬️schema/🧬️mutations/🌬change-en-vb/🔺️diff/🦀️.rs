//! Diff for `change-en-vb`.
use super::ChangeEnVb;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeEnVb, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.en_vb == payload.new_en_vb {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { en_vb: Some(payload.new_en_vb), ..Default::default() })
}
