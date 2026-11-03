//! ↩️ `change-building-ns-q` inverse: restores the previous `nonstructural_q`.

use crate::change_building_ns_q::ChangeBuildingNsQ;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingNsQ, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_ns_q::ChangeBuildingNsQ> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingNsQ, base: &En1998Snapshot) -> Option<ChangeBuildingNsQ> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingNsQ { id: payload.id.clone(), new_nonstructural_q: r0.nonstructural_q })
}
