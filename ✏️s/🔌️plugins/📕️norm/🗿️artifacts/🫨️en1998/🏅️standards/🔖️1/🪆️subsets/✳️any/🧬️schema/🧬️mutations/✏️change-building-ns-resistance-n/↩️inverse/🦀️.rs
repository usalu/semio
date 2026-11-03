//! ↩️ `change-building-ns-resistance-n` inverse: restores the previous `nonstructural_resistance_n`.

use crate::change_building_ns_resistance_n::ChangeBuildingNsResistanceN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingNsResistanceN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_ns_resistance_n::ChangeBuildingNsResistanceN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingNsResistanceN, base: &En1998Snapshot) -> Option<ChangeBuildingNsResistanceN> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingNsResistanceN { id: payload.id.clone(), new_nonstructural_resistance_n: r0.nonstructural_resistance_n })
}
