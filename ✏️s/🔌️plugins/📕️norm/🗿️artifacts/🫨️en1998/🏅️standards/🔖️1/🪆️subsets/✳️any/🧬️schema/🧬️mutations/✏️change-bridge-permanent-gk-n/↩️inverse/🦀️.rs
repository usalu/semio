//! ↩️ `change-bridge-permanent-gk-n` inverse: restores the previous `permanent_gk_n`.

use crate::change_bridge_permanent_gk_n::ChangeBridgePermanentGkN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgePermanentGkN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_permanent_gk_n::ChangeBridgePermanentGkN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgePermanentGkN, base: &En1998Snapshot) -> Option<ChangeBridgePermanentGkN> {
    let r0 = base.bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgePermanentGkN { id: payload.id.clone(), new_permanent_gk_n: r0.permanent_gk_n })
}
