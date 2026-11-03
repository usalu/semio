//! ↩️ `change-roof-ct` inverse: restores the previous `c_t`.

use crate::change_roof_c_t::ChangeRoofCT;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRoofCT, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_roof_c_t::ChangeRoofCT> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRoofCT, base: &En1991Snapshot) -> Option<ChangeRoofCT> {
    let r0 = base.roofs.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRoofCT { id: payload.id.clone(), new_c_t: r0.c_t })
}
