//! ↩️ `change-pier-shear-rd-n` inverse: restores the previous `shear_rd_n`.

use crate::change_pier_shear_rd_n::ChangePierShearRdN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierShearRdN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_shear_rd_n::ChangePierShearRdN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierShearRdN, base: &En1998Snapshot) -> Option<ChangePierShearRdN> {
    let r0 = base.piers.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierShearRdN { id: payload.id.clone(), new_shear_rd_n: r0.shear_rd_n })
}
