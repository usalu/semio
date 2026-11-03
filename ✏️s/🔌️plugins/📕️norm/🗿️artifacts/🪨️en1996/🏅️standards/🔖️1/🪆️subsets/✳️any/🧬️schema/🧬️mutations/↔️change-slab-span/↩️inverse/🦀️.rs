//! ↩️ `change-slab-span` inverse: restores the previous `slab_span_m`.

use crate::change_slab_span::ChangeSlabSpan;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeSlabSpan, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_span::ChangeSlabSpan> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSlabSpan, base: &En1996Snapshot) -> Option<ChangeSlabSpan> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.load_cases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSlabSpan { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_slab_span_m: r1.slab_span_m })
}
