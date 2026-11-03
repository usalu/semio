//! ↩️ `change-tower-base-rot-stiff-nm` inverse: restores the previous `base_rotational_stiffness_nm`.

use crate::change_tower_base_rot_stiff_nm::ChangeTowerBaseRotStiffNm;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerBaseRotStiffNm, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_base_rot_stiff_nm::ChangeTowerBaseRotStiffNm> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTowerBaseRotStiffNm, base: &En1998Snapshot) -> Option<ChangeTowerBaseRotStiffNm> {
    let r0 = base.towers.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTowerBaseRotStiffNm { id: payload.id.clone(), new_base_rotational_stiffness_nm: r0.base_rotational_stiffness_nm })
}
