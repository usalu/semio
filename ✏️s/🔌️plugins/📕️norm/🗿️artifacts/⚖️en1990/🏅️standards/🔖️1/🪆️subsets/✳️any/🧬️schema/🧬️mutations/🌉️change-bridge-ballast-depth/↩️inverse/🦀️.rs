//! ↩️ `change-bridge-ballast-depth` inverse: restores the previous `ballast_depth`.

use crate::change_bridge_ballast_depth::ChangeBridgeBallastDepth;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeBallastDepth, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_ballast_depth::ChangeBridgeBallastDepth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeBallastDepth, base: &En1990Snapshot) -> Option<ChangeBridgeBallastDepth> {
    let r0 = base.bridge_sls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeBallastDepth { id: payload.id.clone(), new_ballast_depth: r0.ballast_depth })
}
