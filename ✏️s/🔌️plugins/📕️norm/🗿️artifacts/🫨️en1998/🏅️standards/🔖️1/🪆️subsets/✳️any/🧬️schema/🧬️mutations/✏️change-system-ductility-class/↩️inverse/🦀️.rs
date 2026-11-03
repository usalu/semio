//! ↩️ `change-system-ductility-class` inverse: restores the previous `ductility_class`.

use crate::change_system_ductility_class::ChangeSystemDuctilityClass;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSystemDuctilityClass, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_system_ductility_class::ChangeSystemDuctilityClass> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSystemDuctilityClass, base: &En1998Snapshot) -> Option<ChangeSystemDuctilityClass> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.systems.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSystemDuctilityClass { building_id: payload.building_id.clone(), id: payload.id.clone(), new_ductility_class: r1.ductility_class })
}
