//! ↩️ `change-pier-section-hm` inverse: restores the previous `section_h_m`.

use crate::change_pier_section_h_m::ChangePierSectionHM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierSectionHM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_section_h_m::ChangePierSectionHM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierSectionHM, base: &En1998Snapshot) -> Option<ChangePierSectionHM> {
    let r0 = base.piers.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierSectionHM { id: payload.id.clone(), new_section_h_m: r0.section_h_m })
}
