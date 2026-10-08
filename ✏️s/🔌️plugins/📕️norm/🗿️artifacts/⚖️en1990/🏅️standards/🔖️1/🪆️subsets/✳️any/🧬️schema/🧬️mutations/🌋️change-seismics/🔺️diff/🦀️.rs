//! 🌋️ `change-seismics` diff — replaces the whole collection: every base row is removed back to front, then every new row is inserted in order.

use super::ChangeSeismics;
use crate::diff::En1990RowEdit as _;
use crate::diff::{En1990Diff, En1990SeismicEdit};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeSeismics, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.seismics == mutation.new_seismics {
        return MutationOutcome::empty().warning("mutation.no-op", "seismics already has this value.");
    }
    let removed = (0..base.seismics.len()).rev().map(|index| En1990SeismicEdit::remove(index, base.seismics[index].id.clone()));
    let inserted = mutation.new_seismics.iter().cloned().enumerate().map(|(index, row)| En1990SeismicEdit::insert(index, row));
    MutationOutcome::new(En1990Diff { seismics: removed.chain(inserted).collect(), ..En1990Diff::default() })
}
