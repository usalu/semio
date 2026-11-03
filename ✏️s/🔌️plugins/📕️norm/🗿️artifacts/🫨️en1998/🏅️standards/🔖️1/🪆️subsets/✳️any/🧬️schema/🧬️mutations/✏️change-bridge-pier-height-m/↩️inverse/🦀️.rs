//! ↩️ `change-bridge-pier-height-m` inverse: restores the previous `pier_height_m`.

use crate::change_bridge_pier_height_m::ChangeBridgePierHeightM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgePierHeightM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_pier_height_m::ChangeBridgePierHeightM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgePierHeightM, base: &En1998Snapshot) -> Option<ChangeBridgePierHeightM> {
    let r0 = base.bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgePierHeightM { id: payload.id.clone(), new_pier_height_m: r0.pier_height_m })
}
