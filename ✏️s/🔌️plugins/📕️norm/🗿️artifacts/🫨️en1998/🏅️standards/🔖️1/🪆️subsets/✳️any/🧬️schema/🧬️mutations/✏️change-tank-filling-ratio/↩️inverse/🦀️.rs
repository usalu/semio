//! ↩️ `change-tank-filling-ratio` inverse: restores the previous `filling_ratio`.

use crate::change_tank_filling_ratio::ChangeTankFillingRatio;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTankFillingRatio, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tank_filling_ratio::ChangeTankFillingRatio> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTankFillingRatio, base: &En1998Snapshot) -> Option<ChangeTankFillingRatio> {
    let r0 = base.tanks.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTankFillingRatio { id: payload.id.clone(), new_filling_ratio: r0.filling_ratio })
}
