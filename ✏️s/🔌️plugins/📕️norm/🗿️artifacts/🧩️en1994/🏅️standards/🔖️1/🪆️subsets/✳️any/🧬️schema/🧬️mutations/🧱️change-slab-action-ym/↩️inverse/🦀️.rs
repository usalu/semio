//! ↩️ `change-slab-action-ym` inverse: restores the previous `y_m`.

use crate::change_slab_action_y_m::ChangeSlabActionYM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn inverse<Op>(payload: &ChangeSlabActionYM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_action_y_m::ChangeSlabActionYM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSlabActionYM, base: &En1994Snapshot) -> Option<ChangeSlabActionYM> {
    let r0 = base.slabs.iter().find(|row| row.id == payload.slab_id)?;
    let r1 = r0.actions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSlabActionYM { slab_id: payload.slab_id.clone(), id: payload.id.clone(), new_y_m: r1.y_m })
}
