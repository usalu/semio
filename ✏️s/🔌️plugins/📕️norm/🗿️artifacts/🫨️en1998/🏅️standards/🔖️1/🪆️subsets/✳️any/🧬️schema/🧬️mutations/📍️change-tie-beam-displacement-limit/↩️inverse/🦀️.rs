//! ↩️ `change-tie-beam-displacement-limit` inverse: restores the previous `displacement-limit`.

use crate::change_tie_beam_displacement_limit::ChangeTieBeamDisplacementLimit;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTieBeamDisplacementLimit, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_beam_displacement_limit::ChangeTieBeamDisplacementLimit> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieBeamDisplacementLimit, base: &En1998Snapshot) -> Option<ChangeTieBeamDisplacementLimit> {
    let r0 = base.tie_beams.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieBeamDisplacementLimit { id: payload.id.clone(), new_displacement_limit: r0.displacement_limit })
}
