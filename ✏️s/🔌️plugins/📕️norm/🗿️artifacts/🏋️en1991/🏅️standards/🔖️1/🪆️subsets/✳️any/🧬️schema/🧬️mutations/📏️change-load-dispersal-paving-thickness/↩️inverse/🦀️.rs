//! ↩️ `change-load-dispersal-paving-thickness` inverse: restores the previous `pavement_thickness`.

use crate::change_load_dispersal_paving_thickness::ChangeLoadDispersalPavingThickness;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeLoadDispersalPavingThickness, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_load_dispersal_paving_thickness::ChangeLoadDispersalPavingThickness> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeLoadDispersalPavingThickness, base: &En1991Snapshot) -> Option<ChangeLoadDispersalPavingThickness> {
    let r0 = base.load_dispersals.iter().find(|row| row.id == payload.id)?;
    Some(ChangeLoadDispersalPavingThickness { id: payload.id.clone(), new_pavement_thickness: r0.pavement_thickness })
}
