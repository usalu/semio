//! ↩️ `change-rail-derailment-loaded-length` inverse: restores the previous `loaded_length`.

use crate::change_rail_derailment_loaded_length::ChangeRailDerailmentLoadedLength;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRailDerailmentLoadedLength, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rail_derailment_loaded_length::ChangeRailDerailmentLoadedLength> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRailDerailmentLoadedLength, base: &En1991Snapshot) -> Option<ChangeRailDerailmentLoadedLength> {
    let r0 = base.rail_derailments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRailDerailmentLoadedLength { id: payload.id.clone(), new_loaded_length: r0.loaded_length })
}
