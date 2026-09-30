//! 🔺️ `change-attachment` sparse diff.

use crate::diff::Din18599Diff;
use crate::mutations::change_attachment::ChangeAttachment;
use crate::Din18599Snapshot;

pub fn diff(payload: &ChangeAttachment, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.attachment == payload.new_attachment {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "attachment already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { attachment: Some(payload.new_attachment), ..Default::default() })
}
