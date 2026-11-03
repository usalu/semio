//! ↩️ `change-beam-action-xm` inverse: restores the previous `x_m`.

use crate::change_beam_action_x_m::ChangeBeamActionXM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn inverse<Op>(payload: &ChangeBeamActionXM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_action_x_m::ChangeBeamActionXM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBeamActionXM, base: &En1994Snapshot) -> Option<ChangeBeamActionXM> {
    let r0 = base.beams.iter().find(|row| row.id == payload.beam_id)?;
    let r1 = r0.actions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBeamActionXM { beam_id: payload.beam_id.clone(), id: payload.id.clone(), new_x_m: r1.x_m })
}
