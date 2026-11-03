//! ↩️ `change-local-fire-fire-diameter` inverse: restores the previous `fire_diameter`.

use crate::change_local_fire_fire_diameter::ChangeLocalFireFireDiameter;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeLocalFireFireDiameter, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_local_fire_fire_diameter::ChangeLocalFireFireDiameter> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeLocalFireFireDiameter, base: &En1991Snapshot) -> Option<ChangeLocalFireFireDiameter> {
    let r0 = base.local_fires.iter().find(|row| row.id == payload.id)?;
    Some(ChangeLocalFireFireDiameter { id: payload.id.clone(), new_fire_diameter: r0.fire_diameter })
}
