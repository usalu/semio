//! ↩️ `change-building-beam-moment-sum-nm` inverse: restores the previous `beam_moment_sum_nm`.

use crate::change_building_beam_moment_sum_nm::ChangeBuildingBeamMomentSumNm;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingBeamMomentSumNm, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_beam_moment_sum_nm::ChangeBuildingBeamMomentSumNm> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingBeamMomentSumNm, base: &En1998Snapshot) -> Option<ChangeBuildingBeamMomentSumNm> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingBeamMomentSumNm { id: payload.id.clone(), new_beam_moment_sum_nm: r0.beam_moment_sum_nm })
}
