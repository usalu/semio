//! ↩️ `change-silo-content-category` inverse: restores the previous `content_category`.

use crate::change_silo_content_category::ChangeSiloContentCategory;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloContentCategory, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_content_category::ChangeSiloContentCategory> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSiloContentCategory, base: &En1998Snapshot) -> Option<ChangeSiloContentCategory> {
    let r0 = base.silos.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSiloContentCategory { id: payload.id.clone(), new_content_category: r0.content_category })
}
