//! ↩️ `change-tie-beam-width` inverse: restores the previous `width`.

use crate::change_tie_beam_width::ChangeTieBeamWidth;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTieBeamWidth, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_beam_width::ChangeTieBeamWidth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieBeamWidth, base: &En1998Snapshot) -> Option<ChangeTieBeamWidth> {
    let r0 = base.tie_beams.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieBeamWidth { id: payload.id.clone(), new_width: r0.width })
}
