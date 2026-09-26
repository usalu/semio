//! 🔺️ `change-method` sparse diff.

use crate::diff::Din18599Diff;
use crate::mutations::change_method::ChangeMethod;
use crate::Din18599Snapshot;

pub fn diff(payload: &ChangeMethod, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.method == payload.new_method {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "method already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { method: Some(payload.new_method), ..Default::default() })
}
