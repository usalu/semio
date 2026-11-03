//! ↩️ `change-building-name` inverse: restores the previous `name`.

use crate::change_building_name::ChangeBuildingName;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingName, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_name::ChangeBuildingName> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingName, base: &En1998Snapshot) -> Option<ChangeBuildingName> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingName { id: payload.id.clone(), new_name: r0.name.clone() })
}
