//! ↩️ `change-construction-load-magnitude` inverse: restores the previous `magnitude`.

use crate::change_construction_load_magnitude::ChangeConstructionLoadMagnitude;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeConstructionLoadMagnitude, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_construction_load_magnitude::ChangeConstructionLoadMagnitude> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeConstructionLoadMagnitude, base: &En1991Snapshot) -> Option<ChangeConstructionLoadMagnitude> {
    let r0 = base.construction_loads.iter().find(|row| row.id == payload.id)?;
    Some(ChangeConstructionLoadMagnitude { id: payload.id.clone(), new_magnitude: r0.magnitude })
}
