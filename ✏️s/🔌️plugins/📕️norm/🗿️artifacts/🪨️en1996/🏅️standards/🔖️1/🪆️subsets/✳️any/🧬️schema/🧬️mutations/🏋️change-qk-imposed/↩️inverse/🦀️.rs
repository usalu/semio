//! ↩️ `change-qk-imposed` inverse: restores the previous `q_k_imposed_pa`.

use crate::change_qk_imposed::ChangeQKImposed;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeQKImposed, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_qk_imposed::ChangeQKImposed> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeQKImposed, base: &En1996Snapshot) -> Option<ChangeQKImposed> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.load_cases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeQKImposed { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_q_k_imposed_pa: r1.q_k_imposed_pa })
}
