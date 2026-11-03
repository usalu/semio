//! ↩️ `change-system-alpha-u-over-alpha1` inverse: restores the previous `alpha_u_over_alpha_1`.

use crate::change_system_alpha_u_over_alpha_1::ChangeSystemAlphaUOverAlpha1;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSystemAlphaUOverAlpha1, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_system_alpha_u_over_alpha_1::ChangeSystemAlphaUOverAlpha1> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSystemAlphaUOverAlpha1, base: &En1998Snapshot) -> Option<ChangeSystemAlphaUOverAlpha1> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.systems.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSystemAlphaUOverAlpha1 { building_id: payload.building_id.clone(), id: payload.id.clone(), new_alpha_u_over_alpha_1: r1.alpha_u_over_alpha_1 })
}
