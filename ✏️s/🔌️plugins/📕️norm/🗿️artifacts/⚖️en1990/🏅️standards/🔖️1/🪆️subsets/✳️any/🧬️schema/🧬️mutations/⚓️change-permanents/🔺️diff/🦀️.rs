//! ⚓️ `change-permanents` diff — replaces the whole collection: every base row leaves, every new row enters after the new row before it.

use super::ChangePermanents;
use crate::diff::{En1990Diff, En1990PermanentAddition, En1990PermanentDelta};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangePermanents, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.permanents == mutation.new_permanents {
        return MutationOutcome::empty().warning("mutation.no-op", "permanents already has this value.");
    }
    let removed = base.permanents.iter().map(|row| row.id.clone()).collect();
    let added = mutation.new_permanents.iter().enumerate().map(|(index, row)| En1990PermanentAddition { after: index.checked_sub(1).map(|previous| mutation.new_permanents[previous].id.clone()), row: row.clone() }).collect();
    MutationOutcome::new(En1990Diff { permanents: En1990PermanentDelta { removed, added, ..En1990PermanentDelta::default() }, ..En1990Diff::default() })
}
