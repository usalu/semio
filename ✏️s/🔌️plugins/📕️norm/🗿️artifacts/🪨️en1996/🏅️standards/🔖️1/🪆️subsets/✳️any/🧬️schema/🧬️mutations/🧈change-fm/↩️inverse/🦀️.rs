//! ↩️ `change-fm` inverse: restores the previous `mortar_strength_pa`.

use crate::change_fm::ChangeFm;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeFm, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fm::ChangeFm> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFm, base: &En1996Snapshot) -> Option<ChangeFm> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFm { id: payload.id.clone(), new_mortar_strength_pa: r0.mortar_strength_pa })
}
