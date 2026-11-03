//! ↩️ `change-tie-beam-depth` inverse: restores the previous `depth`.

use crate::change_tie_beam_depth::ChangeTieBeamDepth;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTieBeamDepth, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_beam_depth::ChangeTieBeamDepth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieBeamDepth, base: &En1998Snapshot) -> Option<ChangeTieBeamDepth> {
    let r0 = base.tie_beams.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieBeamDepth { id: payload.id.clone(), new_depth: r0.depth })
}
