//! ↩️ `change-tie-beam-relative-displacement` inverse: restores the previous `relative-displacement`.

use crate::change_tie_beam_relative_displacement::ChangeTieBeamRelativeDisplacement;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTieBeamRelativeDisplacement, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_beam_relative_displacement::ChangeTieBeamRelativeDisplacement> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieBeamRelativeDisplacement, base: &En1998Snapshot) -> Option<ChangeTieBeamRelativeDisplacement> {
    let r0 = base.tie_beams.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieBeamRelativeDisplacement { id: payload.id.clone(), new_relative_displacement: r0.relative_displacement })
}
