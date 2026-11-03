//! ↩️ `change-hk-earth` inverse: restores the previous `h_k_earth_n`.

use crate::change_hk_earth::ChangeHKEarth;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeHKEarth, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hk_earth::ChangeHKEarth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeHKEarth, base: &En1996Snapshot) -> Option<ChangeHKEarth> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.load_cases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeHKEarth { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_h_k_earth_n: r1.h_k_earth_n })
}
