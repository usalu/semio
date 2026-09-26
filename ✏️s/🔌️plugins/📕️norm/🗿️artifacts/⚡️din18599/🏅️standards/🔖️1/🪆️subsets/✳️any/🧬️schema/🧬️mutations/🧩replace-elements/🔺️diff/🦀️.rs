//! 🔺️ `replace-elements` sparse diff.

use crate::diff::Din18599Diff;
use crate::mutations::replace_elements::ReplaceElements;
use crate::Din18599Snapshot;

pub fn diff(payload: &ReplaceElements, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.elements == payload.new_elements {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "elements already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { elements: Some(crate::diff::Din18599ElementList { values: payload.new_elements.clone() }), ..Default::default() })
}
