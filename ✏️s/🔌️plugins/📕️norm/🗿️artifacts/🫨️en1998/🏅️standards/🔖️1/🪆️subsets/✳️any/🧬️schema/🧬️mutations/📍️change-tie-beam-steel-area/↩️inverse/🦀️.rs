//! ↩️ `change-tie-beam-steel-area` inverse: restores the previous `steel-area`.

use crate::change_tie_beam_steel_area::ChangeTieBeamSteelArea;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTieBeamSteelArea, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_beam_steel_area::ChangeTieBeamSteelArea> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieBeamSteelArea, base: &En1998Snapshot) -> Option<ChangeTieBeamSteelArea> {
    let r0 = base.tie_beams.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieBeamSteelArea { id: payload.id.clone(), new_steel_area: r0.steel_area })
}
