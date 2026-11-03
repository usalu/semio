//! ↩️ `change-building-ns-weight-n` inverse: restores the previous `nonstructural_weight_n`.

use crate::change_building_ns_weight_n::ChangeBuildingNsWeightN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingNsWeightN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_ns_weight_n::ChangeBuildingNsWeightN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingNsWeightN, base: &En1998Snapshot) -> Option<ChangeBuildingNsWeightN> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingNsWeightN { id: payload.id.clone(), new_nonstructural_weight_n: r0.nonstructural_weight_n })
}
