//! ↩️ `change-bridge-pier-shear-rd-n` inverse: restores the previous `pier_shear_rd_n`.

use crate::change_bridge_pier_shear_rd_n::ChangeBridgePierShearRdN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgePierShearRdN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_pier_shear_rd_n::ChangeBridgePierShearRdN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgePierShearRdN, base: &En1998Snapshot) -> Option<ChangeBridgePierShearRdN> {
    let r0 = base.bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgePierShearRdN { id: payload.id.clone(), new_pier_shear_rd_n: r0.pier_shear_rd_n })
}
