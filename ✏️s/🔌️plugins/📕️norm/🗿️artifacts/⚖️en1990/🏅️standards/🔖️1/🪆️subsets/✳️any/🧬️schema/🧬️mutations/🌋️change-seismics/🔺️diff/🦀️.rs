//! 🌋️ `change-seismics` diff — replaces the whole collection: every base row leaves, every new row enters after the new row before it.

use super::ChangeSeismics;
use crate::diff::{En1990Diff, En1990SeismicAddition, En1990SeismicDelta};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeSeismics, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.seismics == mutation.new_seismics {
        return MutationOutcome::empty().warning("mutation.no-op", "seismics already has this value.");
    }
    let removed = base.seismics.iter().map(|row| row.id.clone()).collect();
    let added = mutation.new_seismics.iter().enumerate().map(|(index, row)| En1990SeismicAddition { after: index.checked_sub(1).map(|previous| mutation.new_seismics[previous].id.clone()), row: row.clone() }).collect();
    MutationOutcome::new(En1990Diff { seismics: En1990SeismicDelta { removed, added, ..En1990SeismicDelta::default() }, ..En1990Diff::default() })
}
