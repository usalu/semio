//! ↩️ `change-unit-fb` inverse: restores the previous `f_b_pa`.

use crate::change_unit_fb::ChangeUnitFb;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeUnitFb, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_unit_fb::ChangeUnitFb> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeUnitFb, base: &En1996Snapshot) -> Option<ChangeUnitFb> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeUnitFb { id: payload.id.clone(), new_f_b_pa: r0.f_b_pa })
}
