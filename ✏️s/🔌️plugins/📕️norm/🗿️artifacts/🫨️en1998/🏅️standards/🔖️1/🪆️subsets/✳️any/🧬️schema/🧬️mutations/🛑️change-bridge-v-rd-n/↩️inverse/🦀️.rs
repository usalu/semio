//! ↩️ `change-bridge-v-rd-n` inverse: restores the previous `v_rd_n`.

use crate::change_bridge_v_rd_n::ChangeBridgeVRdN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeVRdN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_v_rd_n::ChangeBridgeVRdN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeVRdN, base: &En1998Snapshot) -> Option<ChangeBridgeVRdN> {
    let r0 = base.bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeVRdN { id: payload.id.clone(), new_v_rd_n: r0.v_rd_n })
}
