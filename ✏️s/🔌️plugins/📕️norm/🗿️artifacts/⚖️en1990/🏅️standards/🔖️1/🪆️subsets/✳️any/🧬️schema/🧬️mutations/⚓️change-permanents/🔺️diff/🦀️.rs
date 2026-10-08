//! ⚓️ `change-permanents` diff — replaces the whole collection: every base row is removed back to front, then every new row is inserted in order.

use super::ChangePermanents;
use crate::diff::En1990RowEdit as _;
use crate::diff::{En1990Diff, En1990PermanentEdit};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangePermanents, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.permanents == mutation.new_permanents {
        return MutationOutcome::empty().warning("mutation.no-op", "permanents already has this value.");
    }
    let removed = (0..base.permanents.len()).rev().map(|index| En1990PermanentEdit::remove(index, base.permanents[index].id.clone()));
    let inserted = mutation.new_permanents.iter().cloned().enumerate().map(|(index, row)| En1990PermanentEdit::insert(index, row));
    MutationOutcome::new(En1990Diff { permanents: removed.chain(inserted).collect(), ..En1990Diff::default() })
}
