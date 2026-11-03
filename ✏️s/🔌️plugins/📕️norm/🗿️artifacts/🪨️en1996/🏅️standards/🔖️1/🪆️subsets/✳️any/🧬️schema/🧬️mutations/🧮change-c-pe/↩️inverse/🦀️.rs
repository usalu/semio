//! ↩️ `change-c-pe` inverse: restores the previous `c_pe`.

use crate::change_c_pe::ChangeCPe;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeCPe, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_c_pe::ChangeCPe> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCPe, base: &En1996Snapshot) -> Option<ChangeCPe> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.load_cases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCPe { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_c_pe: r1.c_pe })
}
