//! ↩️ `change-rail-derailment-situation` inverse: restores the previous `situation`.

use crate::change_rail_derailment_situation::ChangeRailDerailmentSituation;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRailDerailmentSituation, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rail_derailment_situation::ChangeRailDerailmentSituation> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRailDerailmentSituation, base: &En1991Snapshot) -> Option<ChangeRailDerailmentSituation> {
    let r0 = base.rail_derailments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRailDerailmentSituation { id: payload.id.clone(), new_situation: r0.situation.clone() })
}
