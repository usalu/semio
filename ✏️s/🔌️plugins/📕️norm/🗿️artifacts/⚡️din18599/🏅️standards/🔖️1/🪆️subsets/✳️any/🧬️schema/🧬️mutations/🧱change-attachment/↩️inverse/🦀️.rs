//! ↩️ `change-attachment` inverse.

use crate::mutations::change_attachment::ChangeAttachment;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &ChangeAttachment, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeAttachment(ChangeAttachment { new_attachment: base.attachment })]
}
