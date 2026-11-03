//! ↩️ `change-tie-beam-span` inverse: restores the previous `span`.

use crate::change_tie_beam_span::ChangeTieBeamSpan;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTieBeamSpan, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_beam_span::ChangeTieBeamSpan> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieBeamSpan, base: &En1998Snapshot) -> Option<ChangeTieBeamSpan> {
    let r0 = base.tie_beams.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieBeamSpan { id: payload.id.clone(), new_span: r0.span })
}
