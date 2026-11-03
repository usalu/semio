//! ↩️ `change-rail-derailment-offset` inverse: restores the previous `offset`.

use crate::change_rail_derailment_offset::ChangeRailDerailmentOffset;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRailDerailmentOffset, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rail_derailment_offset::ChangeRailDerailmentOffset> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRailDerailmentOffset, base: &En1991Snapshot) -> Option<ChangeRailDerailmentOffset> {
    let r0 = base.rail_derailments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRailDerailmentOffset { id: payload.id.clone(), new_offset: r0.offset })
}
