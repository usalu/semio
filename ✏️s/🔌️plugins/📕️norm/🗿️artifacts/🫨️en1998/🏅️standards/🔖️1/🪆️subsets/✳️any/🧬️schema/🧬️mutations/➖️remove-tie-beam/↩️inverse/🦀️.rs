//! ↩️ `remove-tie-beam` inverse: inserts the removed row back at its position.

use crate::remove_tie_beam::RemoveTieBeam;
use crate::insert_tie_beam::InsertTieBeam;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveTieBeam, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_tie_beam::InsertTieBeam> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveTieBeam, base: &En1998Snapshot) -> Option<InsertTieBeam> {
    let (index, row) = base.tie_beams.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertTieBeam { index: Some(index), tie_beam: row.clone() })
}
