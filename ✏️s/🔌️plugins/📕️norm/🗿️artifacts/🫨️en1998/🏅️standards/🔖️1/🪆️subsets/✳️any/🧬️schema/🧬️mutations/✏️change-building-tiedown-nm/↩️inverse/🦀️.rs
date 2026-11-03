//! ↩️ `change-building-tiedown-nm` inverse: restores the previous `tie_down_moment_nm`.

use crate::change_building_tiedown_nm::ChangeBuildingTiedownNm;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingTiedownNm, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_tiedown_nm::ChangeBuildingTiedownNm> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingTiedownNm, base: &En1998Snapshot) -> Option<ChangeBuildingTiedownNm> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingTiedownNm { id: payload.id.clone(), new_tie_down_moment_nm: r0.tie_down_moment_nm })
}
