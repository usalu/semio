//! ↩️ `change-roof-ce` inverse: restores the previous `c_e`.

use crate::change_roof_c_e::ChangeRoofCE;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRoofCE, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_roof_c_e::ChangeRoofCE> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRoofCE, base: &En1991Snapshot) -> Option<ChangeRoofCE> {
    let r0 = base.roofs.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRoofCE { id: payload.id.clone(), new_c_e: r0.c_e })
}
