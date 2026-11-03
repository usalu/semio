//! ↩️ `change-qp-wind` inverse: restores the previous `q_p_wind_pa`.

use crate::change_qp_wind::ChangeQPWind;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeQPWind, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_qp_wind::ChangeQPWind> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeQPWind, base: &En1996Snapshot) -> Option<ChangeQPWind> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.load_cases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeQPWind { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_q_p_wind_pa: r1.q_p_wind_pa })
}
