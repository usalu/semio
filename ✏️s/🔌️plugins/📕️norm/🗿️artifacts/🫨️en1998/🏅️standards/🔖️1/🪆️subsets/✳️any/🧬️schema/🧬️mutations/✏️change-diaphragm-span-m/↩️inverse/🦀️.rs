//! ↩️ `change-diaphragm-span-m` inverse: restores the previous `span_m`.

use crate::change_diaphragm_span_m::ChangeDiaphragmSpanM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeDiaphragmSpanM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_diaphragm_span_m::ChangeDiaphragmSpanM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDiaphragmSpanM, base: &En1998Snapshot) -> Option<ChangeDiaphragmSpanM> {
    let r0 = base.diaphragms.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDiaphragmSpanM { id: payload.id.clone(), new_span_m: r0.span_m })
}
