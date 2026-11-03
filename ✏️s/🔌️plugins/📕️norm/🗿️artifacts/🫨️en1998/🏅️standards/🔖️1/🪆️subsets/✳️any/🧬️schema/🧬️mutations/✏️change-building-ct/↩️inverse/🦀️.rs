//! ↩️ `change-building-ct` inverse: restores the previous `ct`.

use crate::change_building_ct::ChangeBuildingCt;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingCt, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_ct::ChangeBuildingCt> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingCt, base: &En1998Snapshot) -> Option<ChangeBuildingCt> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingCt { id: payload.id.clone(), new_ct: r0.ct })
}
