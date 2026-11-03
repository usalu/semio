//! ↩️ `change-load-dispersal-slab-thickness` inverse: restores the previous `slab_thickness`.

use crate::change_load_dispersal_slab_thickness::ChangeLoadDispersalSlabThickness;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeLoadDispersalSlabThickness, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_load_dispersal_slab_thickness::ChangeLoadDispersalSlabThickness> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeLoadDispersalSlabThickness, base: &En1991Snapshot) -> Option<ChangeLoadDispersalSlabThickness> {
    let r0 = base.load_dispersals.iter().find(|row| row.id == payload.id)?;
    Some(ChangeLoadDispersalSlabThickness { id: payload.id.clone(), new_slab_thickness: r0.slab_thickness })
}
