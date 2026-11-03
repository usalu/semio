//! ↩️ `change-footbridge-loaded-length` inverse: restores the previous `loaded_length`.

use crate::change_footbridge_loaded_length::ChangeFootbridgeLoadedLength;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFootbridgeLoadedLength, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_footbridge_loaded_length::ChangeFootbridgeLoadedLength> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFootbridgeLoadedLength, base: &En1991Snapshot) -> Option<ChangeFootbridgeLoadedLength> {
    let r0 = base.footbridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFootbridgeLoadedLength { id: payload.id.clone(), new_loaded_length: r0.loaded_length })
}
