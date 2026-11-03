//! ↩️ `change-pier-axial-load-n` inverse: restores the previous `axial_load_n`.

use crate::change_pier_axial_load_n::ChangePierAxialLoadN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierAxialLoadN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_axial_load_n::ChangePierAxialLoadN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierAxialLoadN, base: &En1998Snapshot) -> Option<ChangePierAxialLoadN> {
    let r0 = base.piers.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierAxialLoadN { id: payload.id.clone(), new_axial_load_n: r0.axial_load_n })
}
