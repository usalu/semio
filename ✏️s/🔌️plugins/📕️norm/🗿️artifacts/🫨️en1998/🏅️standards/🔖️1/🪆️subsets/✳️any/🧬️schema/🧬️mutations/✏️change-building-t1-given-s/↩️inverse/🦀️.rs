//! ↩️ `change-building-t1-given-s` inverse: restores the previous `t1_given_s`.

use crate::change_building_t1_given_s::ChangeBuildingT1GivenS;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingT1GivenS, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_t1_given_s::ChangeBuildingT1GivenS> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingT1GivenS, base: &En1998Snapshot) -> Option<ChangeBuildingT1GivenS> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingT1GivenS { id: payload.id.clone(), new_t1_given_s: r0.t1_given_s })
}
