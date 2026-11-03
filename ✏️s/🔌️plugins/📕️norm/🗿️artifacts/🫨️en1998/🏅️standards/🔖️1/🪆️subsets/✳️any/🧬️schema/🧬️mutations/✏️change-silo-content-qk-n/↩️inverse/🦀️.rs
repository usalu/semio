//! ↩️ `change-silo-content-qk-n` inverse: restores the previous `content_qk_n`.

use crate::change_silo_content_qk_n::ChangeSiloContentQkN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloContentQkN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_content_qk_n::ChangeSiloContentQkN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSiloContentQkN, base: &En1998Snapshot) -> Option<ChangeSiloContentQkN> {
    let r0 = base.silos.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSiloContentQkN { id: payload.id.clone(), new_content_qk_n: r0.content_qk_n })
}
