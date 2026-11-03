//! ↩️ `change-rail-derailment-assumed-line-load` inverse: restores the previous `assumed_line_load`.

use crate::change_rail_derailment_assumed_line_load::ChangeRailDerailmentAssumedLineLoad;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRailDerailmentAssumedLineLoad, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rail_derailment_assumed_line_load::ChangeRailDerailmentAssumedLineLoad> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRailDerailmentAssumedLineLoad, base: &En1991Snapshot) -> Option<ChangeRailDerailmentAssumedLineLoad> {
    let r0 = base.rail_derailments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRailDerailmentAssumedLineLoad { id: payload.id.clone(), new_assumed_line_load: r0.assumed_line_load })
}
