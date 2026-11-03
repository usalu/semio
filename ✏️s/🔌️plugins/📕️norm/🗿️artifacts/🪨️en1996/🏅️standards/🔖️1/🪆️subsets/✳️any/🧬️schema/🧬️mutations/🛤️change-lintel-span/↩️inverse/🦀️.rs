//! ↩️ `change-lintel-span` inverse: restores the previous `span_m`.

use crate::change_lintel_span::ChangeLintelSpan;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeLintelSpan, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lintel_span::ChangeLintelSpan> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeLintelSpan, base: &En1996Snapshot) -> Option<ChangeLintelSpan> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.lintels.iter().find(|row| row.id == payload.id)?;
    Some(ChangeLintelSpan { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_span_m: r1.span_m })
}
