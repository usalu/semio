//! ➖️ `remove-foundation` diff — removes the row at the index, guarded by the row's own id; an index past the collection's end is a `mutation.target-missing`.

use super::RemoveFoundation;
use crate::diff::En1998RowEdit as _;
use crate::diff::{En1998Diff, En1998FoundationEdit};
use crate::En1998Snapshot;

pub fn diff(payload: &RemoveFoundation, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(row) = base.foundations.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("foundation #{}", payload.index), [payload.index.to_string()]);
    };
    protocol::MutationOutcome::new(En1998Diff { foundations: vec![En1998FoundationEdit::remove(payload.index, row.id.clone())], ..Default::default() })
}
