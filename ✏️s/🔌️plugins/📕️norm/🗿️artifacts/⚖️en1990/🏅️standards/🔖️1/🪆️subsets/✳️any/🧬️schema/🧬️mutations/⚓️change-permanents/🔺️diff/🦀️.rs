//! ⚓️ `change-permanents` diff — replaces the whole collection: every base row leaves from its base index, every new row enters at its index.

use super::ChangePermanents;
use crate::diff::{En1990Diff, En1990PermanentDelta, En1990PermanentInsertion, En1990PermanentRemoval};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangePermanents, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.permanents == mutation.new_permanents {
        return MutationOutcome::empty().warning("mutation.no-op", "permanents already has this value.");
    }
    let removed = base.permanents.iter().enumerate().map(|(index, row)| En1990PermanentRemoval { id: row.id.clone(), index }).collect();
    let inserted = mutation.new_permanents.iter().enumerate().map(|(index, row)| En1990PermanentInsertion { index, row: row.clone() }).collect();
    MutationOutcome::new(En1990Diff { permanents: En1990PermanentDelta { removed, inserted, ..En1990PermanentDelta::default() }, ..En1990Diff::default() })
}
