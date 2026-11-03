//! ↩️ `change-tank-content-qk-n` inverse: restores the previous `content_qk_n`.

use crate::change_tank_content_qk_n::ChangeTankContentQkN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTankContentQkN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tank_content_qk_n::ChangeTankContentQkN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTankContentQkN, base: &En1998Snapshot) -> Option<ChangeTankContentQkN> {
    let r0 = base.tanks.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTankContentQkN { id: payload.id.clone(), new_content_qk_n: r0.content_qk_n })
}
