//! ↩️ `change-building-multi-resisting-systems` inverse: restores the previous `multiple_resisting_systems`.

use crate::change_building_multi_resisting_systems::ChangeBuildingMultiResistingSystems;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingMultiResistingSystems, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_multi_resisting_systems::ChangeBuildingMultiResistingSystems> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingMultiResistingSystems, base: &En1998Snapshot) -> Option<ChangeBuildingMultiResistingSystems> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingMultiResistingSystems { id: payload.id.clone(), new_multiple_resisting_systems: r0.multiple_resisting_systems })
}
