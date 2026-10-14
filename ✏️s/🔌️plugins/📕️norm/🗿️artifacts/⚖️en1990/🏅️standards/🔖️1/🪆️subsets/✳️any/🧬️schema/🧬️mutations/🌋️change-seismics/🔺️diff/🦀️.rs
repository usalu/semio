//! 🌋️ `change-seismics` diff — replaces the whole collection: every base row leaves from its base index, every new row enters at its index.

use crate::change_seismics::ChangeSeismics;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990SeismicDelta, En1990SeismicInsertion, En1990SeismicRemoval};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeSeismics, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.seismics == mutation.new_seismics {
        return MutationOutcome::empty().warning("mutation.no-op", "seismics already has this value.");
    }
    let removed = base.seismics.iter().enumerate().map(|(index, row)| En1990SeismicRemoval { id: row.id.clone(), index }).collect();
    let inserted = mutation.new_seismics.iter().enumerate().map(|(index, row)| En1990SeismicInsertion { index, row: row.clone() }).collect();
    MutationOutcome::new(En1990Diff { seismics: En1990SeismicDelta { removed, inserted, ..En1990SeismicDelta::default() }, ..En1990Diff::default() })
}
