//! ↩️ `change-prestress-loss` inverse: restores the previous `loss_ratio`.

use crate::change_prestress_loss::ChangePrestressLoss;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangePrestressLoss, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_prestress_loss::ChangePrestressLoss> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePrestressLoss, base: &En1996Snapshot) -> Option<ChangePrestressLoss> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.prestress.iter().find(|row| row.id == payload.id)?;
    Some(ChangePrestressLoss { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_loss_ratio: r1.loss_ratio })
}
