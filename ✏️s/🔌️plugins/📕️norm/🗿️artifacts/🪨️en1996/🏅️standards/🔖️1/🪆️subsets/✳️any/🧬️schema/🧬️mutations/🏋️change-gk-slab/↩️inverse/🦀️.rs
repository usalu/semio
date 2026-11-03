//! ↩️ `change-gk-slab` inverse: restores the previous `g_k_slab_n`.

use crate::change_gk_slab::ChangeGKSlab;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeGKSlab, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_gk_slab::ChangeGKSlab> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeGKSlab, base: &En1996Snapshot) -> Option<ChangeGKSlab> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.load_cases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeGKSlab { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_g_k_slab_n: r1.g_k_slab_n })
}
