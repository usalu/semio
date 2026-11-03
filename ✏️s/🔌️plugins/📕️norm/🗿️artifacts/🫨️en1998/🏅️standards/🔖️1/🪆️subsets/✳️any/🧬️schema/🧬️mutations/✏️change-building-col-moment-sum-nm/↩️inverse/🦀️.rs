//! ↩️ `change-building-col-moment-sum-nm` inverse: restores the previous `column_moment_sum_nm`.

use crate::change_building_col_moment_sum_nm::ChangeBuildingColMomentSumNm;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingColMomentSumNm, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_col_moment_sum_nm::ChangeBuildingColMomentSumNm> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingColMomentSumNm, base: &En1998Snapshot) -> Option<ChangeBuildingColMomentSumNm> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingColMomentSumNm { id: payload.id.clone(), new_column_moment_sum_nm: r0.column_moment_sum_nm })
}
