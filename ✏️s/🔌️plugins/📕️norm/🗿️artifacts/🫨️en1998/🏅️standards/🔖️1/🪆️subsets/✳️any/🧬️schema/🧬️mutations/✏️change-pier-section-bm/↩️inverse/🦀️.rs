//! ↩️ `change-pier-section-bm` inverse: restores the previous `section_b_m`.

use crate::change_pier_section_b_m::ChangePierSectionBM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierSectionBM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_section_b_m::ChangePierSectionBM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierSectionBM, base: &En1998Snapshot) -> Option<ChangePierSectionBM> {
    let r0 = base.piers.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierSectionBM { id: payload.id.clone(), new_section_b_m: r0.section_b_m })
}
