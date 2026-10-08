//! 🔺️ `replace-elements` sparse diff.

use crate::mutations::replace_elements::ReplaceElements;
use crate::Din18599Snapshot;
use crate::diff::{Din18599Diff, Din18599ElementsRows};

pub fn diff(payload: &ReplaceElements, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if base.elements == payload.new_elements {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "elements already has this value.");
    }
    if let Some((_, row)) = payload.new_elements.iter().enumerate().find(|(at, row)| payload.new_elements[..*at].iter().any(|earlier| earlier.id == row.id)) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Element id {} appears twice.", row.id), [row.id.clone()]);
    }
    protocol::MutationOutcome::new(Din18599Diff { elements: Some(Din18599ElementsRows::setting(&base.elements, &payload.new_elements)), ..Default::default() })
}
