//! ↩️ `change-building-t1-method` inverse: restores the previous `t1_method`.

use crate::change_building_t1_method::ChangeBuildingT1Method;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingT1Method, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_t1_method::ChangeBuildingT1Method> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingT1Method, base: &En1998Snapshot) -> Option<ChangeBuildingT1Method> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingT1Method { id: payload.id.clone(), new_t1_method: r0.t1_method })
}
