//! ↩️ `change-local-fire-ceiling-height` inverse: restores the previous `ceiling_height`.

use crate::change_local_fire_ceiling_height::ChangeLocalFireCeilingHeight;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeLocalFireCeilingHeight, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_local_fire_ceiling_height::ChangeLocalFireCeilingHeight> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeLocalFireCeilingHeight, base: &En1991Snapshot) -> Option<ChangeLocalFireCeilingHeight> {
    let r0 = base.local_fires.iter().find(|row| row.id == payload.id)?;
    Some(ChangeLocalFireCeilingHeight { id: payload.id.clone(), new_ceiling_height: r0.ceiling_height })
}
