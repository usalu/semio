//! ↩️ `insert-tie-beam` inverse: removes the inserted row again.

use crate::insert_tie_beam::InsertTieBeam;
use crate::remove_tie_beam::RemoveTieBeam;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertTieBeam, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_tie_beam::RemoveTieBeam> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertTieBeam, base: &En1998Snapshot) -> Option<RemoveTieBeam> {
    Some(RemoveTieBeam { id: payload.tie_beam.id.clone() })
}
